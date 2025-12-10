import type { Dispatch, SetStateAction } from "react";

import { CoreFFI } from "shared/shared";
import { BincodeDeserializer, BincodeSerializer } from "shared_types/bincode";
import {
    ClockOperationVariantStart,
    ClockOperationVariantStop,
    ClockOutputVariantTick,
    ClockOutputVariantStopped,
    Effect,
    EffectVariantRender,
    EffectVariantWallClock,
    Event,
    Request,
    ViewModel,
} from "shared_types/app";

import { start, stop } from "./interval";
import init_core from "shared/shared";

type Output = ClockOutputVariantTick | ClockOutputVariantStopped;

export class Core {
    core: CoreFFI;
    setViewModel: Dispatch<SetStateAction<ViewModel>>;

    constructor(setViewModel: Dispatch<SetStateAction<ViewModel>>) {
        this.setViewModel = setViewModel;
    }

    initialize(should_load: boolean) {
        if (!this.core) {
            const load = should_load ? init_core() : Promise.resolve();

            console.log("Loading WASM.");
            load.then(() => {
                console.log("Creating FFI instance.");
                this.core = new CoreFFI();
                console.log("FFI loaded.");
                this.setViewModel(this.view());
            }).catch((error) => {
                console.error("Failed to initialize wasm core:", error);
            });
        }
    }

    update(event: Event) {
        if (!this.core) {
            throw new Error("Core not initialized. Call initialize() first.");
        }

        const requests = this.processEvent(event);
        this.processEffects(requests);
    }

    view(): ViewModel {
        if (!this.core) {
            throw new Error("Core not initialized. Call initialize() first.");
        }

        const bytes = this.core.view();
        return ViewModel.deserialize(new BincodeDeserializer(bytes));
    }

    // Internal

    private processEffects(requests: Request[]) {
        for (const { id, effect } of requests) {
            this.processEffect(id, effect);
        }
    }

    private processEffect(id: number, effect: Effect) {
        if (effect.constructor == EffectVariantRender) {
            console.log("Render effect:", effect);
            const view = this.view();

            console.log("New view model:", view);

            this.setViewModel(view);
        } else if (effect.constructor == EffectVariantWallClock) {
            const clockOperation = (effect as EffectVariantWallClock).value;

            console.log("Clock effect:", clockOperation);

            if (clockOperation.constructor == ClockOperationVariantStart) {
                const interval = (clockOperation as ClockOperationVariantStart)
                    .value;

                start(id, interval, () => {
                    console.log("Timer tick");

                    const requests = this.handleResponse(
                        id,
                        new ClockOutputVariantTick(),
                    );
                    this.processEffects(requests);
                });
            } else if (
                clockOperation.constructor == ClockOperationVariantStop
            ) {
                const id = stop();

                if (id !== null) {
                    const requests = this.handleResponse(
                        id,
                        new ClockOutputVariantStopped(),
                    );
                    this.processEffects(requests);
                }
            }
        }
    }

    // Serialization shims

    private processEvent(event: Event): Request[] {
        const serializer = new BincodeSerializer();
        event.serialize(serializer);

        const requestBytes = this.core.update(serializer.getBytes());

        return this.deserializeRequests(requestBytes);
    }

    private handleResponse(id: number, value: Output): Request[] {
        const serializer = new BincodeSerializer();
        value.serialize(serializer);

        const request_bytes = this.core.resolve(id, serializer.getBytes());

        return this.deserializeRequests(request_bytes);
    }

    private deserializeRequests(bytes: Uint8Array): Request[] {
        const deserializer = new BincodeDeserializer(bytes);
        const len = deserializer.deserializeLen();
        const requests: Request[] = [];
        for (let i = 0; i < len; i++) {
            const request = Request.deserialize(deserializer);
            requests.push(request);
        }
        return requests;
    }
}
