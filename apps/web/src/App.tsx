import { useEffect, useRef, useState } from "react";
import "./App.css";
import init, { DesignRenderer } from "design-app-renderer";
import type { DesignCommand, DraftLayer } from "design-app-renderer-types";
import { useAtom } from "jotai";
import { wasmLoadedAtom } from "./store/app";

async function initializeRenderer() {
  const renderer = await DesignRenderer.create("preview");
  await renderer.run();
  return renderer;
}

function App() {
  const renderer = useRef<DesignRenderer | null>(null);
  const rendererLoaded = useRef(false);
  const [wasmLoaded, setWasmLoaded] = useAtom(wasmLoadedAtom);

  // Handle initializing WASM + Renderer
  useEffect(() => {
    async function startRenderer() {
      renderer.current = await initializeRenderer();
      rendererLoaded.current = true;
    }
    async function runWasm() {
      await init();
      setWasmLoaded(true);
    }

    // WASM isn't loaded? Initialize it
    if (!wasmLoaded) runWasm();
    // Loaded and haven't run renderer yet? Initialize it and run render loop
    if (wasmLoaded && !rendererLoaded.current) startRenderer();
  }, [wasmLoaded]);

  const dispatch = (command: DesignCommand) => {
    if (!renderer.current) return;

    renderer.current.dispatch(command);
  };

  const handleCreateLayer = () => {
    const width = Math.round(Math.random() * 420);
    const height = Math.round(Math.random() * 420);
    dispatch({
      AddLayer: {
        layer: {
          name: "Test",
          kind: "Rect",
          position: [0, 0],
          width,
          height,
          opacity: 1.0,
          parent: null,
          visible: true,
          items: [],
        },
      },
    });
  };

  return (
    <>
      <div style={{ position: "fixed", top: 32, right: 32 }}>
        <button onClick={handleCreateLayer}>Create layer</button>
      </div>
      <canvas id="preview" width={800} height={600} />
    </>
  );
}

export default App;
