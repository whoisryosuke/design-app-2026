import { useEffect, useState } from "react";
import heroImg from "./assets/hero.png";
import reactLogo from "./assets/react.svg";
import viteLogo from "./assets/vite.svg";
import "./App.css";
import init, { run } from "design-app-renderer";

let started: Promise<void> | null = null;

function runWasm() {
  if (!started) {
    started = init().then(() => run("preview"));
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
