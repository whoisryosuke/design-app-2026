import { useEffect, useRef, useState } from "react";
import heroImg from "./assets/hero.png";
import reactLogo from "./assets/react.svg";
import viteLogo from "./assets/vite.svg";
import "./App.css";
import init, { DesignRenderer } from "design-app-renderer";

async function initializeRenderer() {
  const renderer = await DesignRenderer.create("preview");
  await renderer.run();
  return renderer;
}

function App() {
  const [count, setCount] = useState(0);
  const [wasmLoaded, setWasmLoaded] = useState(false);
  const renderer = useRef<DesignRenderer | null>(null);
  const rendererLoaded = useRef(false);

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

  // const dispatch = (command: DesignCommand) => {
  //   if (!renderer.current) return;

  //   renderer.current.dispatch();
  // };

  return (
    <>
      <canvas id="preview" width={800} height={600} />
    </>
  );
}

export default App;
