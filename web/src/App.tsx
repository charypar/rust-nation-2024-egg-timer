import { useState } from "react";

import { EventVariantHello } from "shared_types/types/shared_types";
import { update, view } from "./app/core";

function App() {
  const [viewModel, setView] = useState(view());

  return (
    <main className="card bg-base-100 mx-auto my-10 w-96">
      <div className="card prose p-4 shadow-xl">
        <h1>{viewModel.message}</h1>
        <p className="text-center">
          <button
            className="btn"
            onClick={() => update(new EventVariantHello(), setView)}
          >
            Say hello
          </button>
        </p>
      </div>
    </main>
  );
}

export default App;
