import type { Dispatch, SetStateAction } from "react";

import { handle_response, process_event, view as rawView } from "shared/shared";
import {
  BincodeDeserializer,
  BincodeSerializer,
} from "shared_types/bincode/mod";
import {
  ClockOperationVariantStart,
  ClockOperationVariantStop,
  Effect,
  EffectVariantRender,
  EffectVariantWallClock,
  Event,
  Request,
  ViewModel,
} from "shared_types/types/shared_types";
import { start, stop } from "./interval";
import { ClockOutputVariantTick } from "shared_types/types/shared_types";
import { ClockOutputVariantStopped } from "shared_types/types/shared_types";

export function update(
  event: Event,
  callback: Dispatch<SetStateAction<ViewModel>>,
) {
  console.log("Update with event:", event);

  const requests = processEvent(event);
  processEffects(requests, callback);
}

// Internal

function processEffects(
  requests: Request[],
  callback: Dispatch<SetStateAction<ViewModel>>,
) {
  for (const { uuid, effect } of requests) {
    processEffect(uuid, effect, callback);
  }
}

function processEffect(
  uuid: number[],
  effect: Effect,
  callback: Dispatch<SetStateAction<ViewModel>>,
) {
  if (effect.constructor == EffectVariantRender) {
    console.log("Render effect:", effect);

    callback(view());
  } else if (effect.constructor == EffectVariantWallClock) {
    const clockOperation = (effect as EffectVariantWallClock).value;

    console.log("Clock effect:", clockOperation);

    if (clockOperation.constructor == ClockOperationVariantStart) {
      const interval = (clockOperation as ClockOperationVariantStart).value;

      start(uuid, interval, () => {
        console.log("Timer tick");

        const requests = handleResponse(uuid, new ClockOutputVariantTick());
        processEffects(requests, callback);
      });
    } else if (clockOperation.constructor == ClockOperationVariantStop) {
      const id = stop();

      if (id !== null) {
        const requests = handleResponse(id, new ClockOutputVariantStopped());
        processEffects(requests, callback);
      }
    }
  }
}

// Serialization shims

function processEvent(event: Event): Request[] {
  const serializer = new BincodeSerializer();
  event.serialize(serializer);

  const requestBytes = process_event(serializer.getBytes());

  return deserializeRequests(requestBytes);
}

type Output = ClockOutputVariantTick | ClockOutputVariantStopped;

function handleResponse(uuid: number[], value: Output): Request[] {
  const serializer = new BincodeSerializer();
  value.serialize(serializer);

  const request_bytes = handle_response(
    new Uint8Array(uuid),
    serializer.getBytes(),
  );

  return deserializeRequests(request_bytes);
}

export function view(): ViewModel {
  const bytes = rawView();

  return ViewModel.deserialize(new BincodeDeserializer(bytes));
}

function deserializeRequests(bytes: Uint8Array): Request[] {
  const deserializer = new BincodeDeserializer(bytes);
  const len = deserializer.deserializeLen();
  const requests: Request[] = [];
  for (let i = 0; i < len; i++) {
    const request = Request.deserialize(deserializer);
    requests.push(request);
  }
  return requests;
}
