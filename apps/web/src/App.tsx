import { useEffect, useState } from "react";
import heroImg from "./assets/hero.png";
import reactLogo from "./assets/react.svg";
import viteLogo from "./assets/vite.svg";
import "./App.css";
import init, { DesignRenderer } from "design-app-renderer";

let started: Promise<void> | null = null;

async function initializeRenderer() {
  const renderer = await DesignRenderer.create("preview");
  await renderer.run();
}

function runWasm() {
  if (!started) {
    started = init().then(async () => await initializeRenderer());
  }
  return started;
}

function App() {
  const [count, setCount] = useState(0);

  useEffect(() => {
    runWasm();
  }, []);

  return (
    <>
      <canvas id="preview" width={800} height={600} />
    </>
  );
}

export default App;
