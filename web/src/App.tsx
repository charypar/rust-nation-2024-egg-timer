import { useState } from "react";

import { EventVariantHello } from "shared_types/types/shared_types";
import { update, view } from "./app/core";

function App() {
  const [_viewModel, setView] = useState(view());

  return (
    <main className="mx-auto w-96 py-36">
      <div className="card bg-base-300 my-4 p-4 shadow-xl">
        <div className="flex flex-col items-center">
          <Clock progress={38} minutes={3} seconds={10} />
          <p className="text-primary my-6 flex justify-evenly self-stretch text-center">
            <button
              className="btn btn-circle btn-secondary"
              onClick={() => update(new EventVariantHello(), setView)}
            >
              -10
            </button>
            <button
              className="btn btn-circle btn-success"
              onClick={() => update(new EventVariantHello(), setView)}
            >
              run
            </button>
            <button
              className="btn btn-circle btn-error"
              disabled
              onClick={() => update(new EventVariantHello(), setView)}
            >
              reset
            </button>
            <button
              className="btn btn-circle btn-secondary"
              onClick={() => update(new EventVariantHello(), setView)}
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
};

function Clock({ progress, minutes, seconds }: ClockProps) {
  return (
    <div
      className="radial-progress text-base-content m-2 "
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
            <span style={{ "--value": minutes } as React.CSSProperties}></span>
          </span>
          min
        </div>
        <div>
          <span className="countdown font-mono text-5xl">
            <span style={{ "--value": seconds } as React.CSSProperties}></span>
          </span>
          sec
        </div>
      </div>
    </div>
  );
}

export default App;
