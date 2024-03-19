import { useEffect, useRef, useState } from "react";
import "./App.css";

import init_core from "shared/shared";
import { EventVariantHello, ViewModel } from "shared_types/types/shared_types";
import { update, view } from "./app/core";

function App() {
  const [viewModel, setView] = useState(new ViewModel(""));

  const initialized = useRef(false);
  useEffect(() => {
    if (!initialized.current) {
      initialized.current = true;

      init_core().then(() => setView(view()));
    }
  }, []);

  console.log("App!");

  return (
    <>
      <h1>{viewModel.message}</h1>
      <div className="card">
        <button onClick={() => update(new EventVariantHello(), setView)}>
          Say hello
        </button>
      </div>
    </>
  );
}

export default App;
