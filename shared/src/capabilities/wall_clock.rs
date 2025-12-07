use std::marker::Send;

use crux_core::{capability::Operation, command::StreamBuilder, Command, Request};
use facet::Facet;
use futures::Stream;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Facet, Serialize, Deserialize)]
#[repr(C)]
pub enum ClockOperation {
    Start(u32),
    Stop,
}

#[derive(Debug, PartialEq, Facet, Serialize, Deserialize)]
#[repr(C)]
pub enum ClockOutput {
    Tick,
    Stopped,
}

impl Operation for ClockOperation {
    type Output = ClockOutput;
}

pub fn start<Effect, Event>(
    interval: u32,
) -> StreamBuilder<Effect, Event, impl Stream<Item = ClockOutput>>
where
    Effect: From<Request<ClockOperation>> + Send + 'static,
    Event: Send + 'static,
{
    Command::stream_from_shell(ClockOperation::Start(interval))
}

pub fn stop<Effect, Event>() -> Command<Effect, Event>
where
    Effect: Send + From<crux_core::Request<ClockOperation>> + 'static,
    Event: Send + 'static,
{
    Command::notify_shell(ClockOperation::Stop).build()
}
