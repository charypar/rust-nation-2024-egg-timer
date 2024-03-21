let interval: number;
let subscriptionId: number[] | null = null;

export function start(
  effectId: number[],
  intervalSec: number,
  callback: () => void,
) {
  console.log("Starting clock");

  subscriptionId = effectId;
  interval = setInterval(callback, intervalSec * 1000);
}

export function stop(): number[] | null {
  console.log("Stopping clock", subscriptionId);

  clearInterval(interval);

  const id = subscriptionId;
  subscriptionId = null;

  return id;
}
