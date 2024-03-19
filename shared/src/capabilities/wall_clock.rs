use crux_core::{
    capability::{CapabilityContext, Operation},
    Capability,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

struct WallClock<Ev> {
    context: CapabilityContext<ClockOperation, Ev>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ClockOperation {
    Start(usize),
    Stop,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ClockOutput {
    Tick,
    Stopped,
}

impl Operation for ClockOperation {
    type Output = ClockOutput;
}

impl<Ev> WallClock<Ev> {
    pub fn new(context: CapabilityContext<ClockOperation, Ev>) -> Self {
        Self { context }
    }

    pub fn start<E>(&self, interval: usize, event: E)
    where
        E: Fn() -> Ev + Send + Sync + 'static,
        Ev: 'static,
    {
        self.context.spawn({
            let ctx = self.context.clone();

            async move {
                let mut subscription = ctx.stream_from_shell(ClockOperation::Start(interval));

                while let Some(ClockOutput::Tick) = subscription.next().await {
                    ctx.update_app(event());
                }
            }
        });
    }
}

impl<Ev> Capability<Ev> for WallClock<Ev> {
    type Operation = ClockOperation;
    type MappedSelf<MappedEv> = WallClock<MappedEv>;

    fn map_event<F, NewEv>(&self, f: F) -> Self::MappedSelf<NewEv>
    where
        F: Fn(NewEv) -> Ev + Send + Sync + Copy + 'static,
        Ev: 'static,
        NewEv: 'static + Send,
    {
        WallClock::new(self.context.map_event(f))
    }
}
