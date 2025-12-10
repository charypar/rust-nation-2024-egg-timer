import clsx from "clsx";
import { useCallback, useEffect, useRef, useState } from "react";

import {
    Event,
    EventVariantDecrease,
    EventVariantIncrease,
    EventVariantReset,
    EventVariantToggleRunning,
    ViewModel,
} from "shared_types/app";
import { Core } from "./app/core";

function App() {
    const [viewModel, setView] = useState(
        new ViewModel(0, 0, 0, false, false, true, true, false),
    );

    const core = useRef(new Core(setView));
    const update = useCallback(
        (event: Event) => {
            core.current.update(event);
        },
        [core],
    );

    // Initialize
    useEffect(
        () => {
            console.log("Initializing core!");
            core.current.initialize(true);
        },
        // eslint-disable-next-line react-hooks/exhaustive-deps
        /*once*/ [core],
    );

    return (
        <main className="mx-auto w-96 py-36">
            <div className="card bg-base-300 my-4 p-4 shadow-xl">
                <div className="flex flex-col items-center">
                    <Clock
                        progress={viewModel.percentage}
                        minutes={viewModel.minutes}
                        seconds={viewModel.seconds}
                        finished={viewModel.finished}
                    />
                    <p className="text-primary my-6 flex justify-evenly self-stretch text-center">
                        <button
                            className="btn btn-circle btn-secondary"
                            disabled={!viewModel.canEdit}
                            onClick={() => update(new EventVariantDecrease())}
                        >
                            -10
                        </button>
                        <button
                            className="btn btn-circle btn-success"
                            disabled={!viewModel.canToggleRunnning}
                            onClick={() =>
                                update(new EventVariantToggleRunning())
                            }
                        >
                            {viewModel.running ? "pause" : "run"}
                        </button>
                        <button
                            className="btn btn-circle btn-error"
                            disabled={!viewModel.canReset}
                            onClick={() => update(new EventVariantReset())}
                        >
                            reset
                        </button>
                        <button
                            className="btn btn-circle btn-secondary"
                            disabled={!viewModel.canEdit}
                            onClick={() => update(new EventVariantIncrease())}
                        >
                            +10
                        </button>
                    </p>
                </div>
            </div>
        </main>
    );
}

type ClockProps = {
    progress: number;
    minutes: number;
    seconds: number;
    finished: boolean;
};

function Clock({ progress, minutes, seconds, finished }: ClockProps) {
    return (
        <div
            className={clsx(
                "radial-progress m-2",
                finished ? "text-primary animate-bounce" : "text-base-content",
            )}
            style={
                {
                    "--value": progress,
                    "--size": "20rem",
                    "--thickness": "20px",
                } as React.CSSProperties
            }
            role="progressbar"
        >
            <div className="flex gap-5">
                <div>
                    <span className="countdown font-mono text-5xl">
                        <span
                            style={
                                { "--value": minutes } as React.CSSProperties
                            }
                        ></span>
                    </span>
                    min
                </div>
                <div>
                    <span className="countdown font-mono text-5xl">
                        <span
                            style={
                                { "--value": seconds } as React.CSSProperties
                            }
                        ></span>
                    </span>
                    sec
                </div>
            </div>
        </div>
    );
}

export default App;
