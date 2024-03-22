import type { Dispatch, SetStateAction } from "react";

import { handle_response, process_event, view as rawView } from "shared/shared";
import {
  BincodeDeserializer,
  BincodeSerializer,
} from "shared_types/bincode/mod";
import {
  Effect,
  EffectVariantRender,
  Event,
  Request,
  ViewModel,
} from "shared_types/types/shared_types";

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
  _uuid: number[],
  effect: Effect,
  callback: Dispatch<SetStateAction<ViewModel>>,
) {
  if (effect.constructor == EffectVariantRender) {
    console.log("Render effect:", effect);

    callback(view());
  }
}

// Serialization shims

function processEvent(event: Event): Request[] {
  const serializer = new BincodeSerializer();
  event.serialize(serializer);

  const requestBytes = process_event(serializer.getBytes());

  return deserializeRequests(requestBytes);
}

type Output = { serialize(serializer: BincodeSerializer): void };

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
