import type { Dispatch, SetStateAction } from "react";

import { process_event, view as rawView } from "shared/shared";
import type { Effect, Event } from "shared_types/types/shared_types";
import {
  EffectVariantRender,
  ViewModel,
  Request,
} from "shared_types/types/shared_types";
import {
  BincodeSerializer,
  BincodeDeserializer,
} from "shared_types/bincode/mod";

export function update(
  event: Event,
  callback: Dispatch<SetStateAction<ViewModel>>,
) {
  console.log("Update with event:", event);

  const serializer = new BincodeSerializer();
  event.serialize(serializer);

  const effects = process_event(serializer.getBytes());

  const requests = deserializeRequests(effects);
  for (const { uuid, effect } of requests) {
    processEffect(uuid, effect, callback);
  }
}

export function view() {
  const bytes = rawView();
  return deserializeView(bytes);
}

function processEffect(
  _uuid: number[],
  effect: Effect,
  callback: Dispatch<SetStateAction<ViewModel>>,
) {
  console.log("Processing effect:", effect);

  switch (effect.constructor) {
    case EffectVariantRender: {
      callback(deserializeView(rawView()));
      break;
    }
  }
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

function deserializeView(bytes: Uint8Array): ViewModel {
  return ViewModel.deserialize(new BincodeDeserializer(bytes));
}
