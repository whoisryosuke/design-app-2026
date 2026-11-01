import { useEffect, useRef } from "react";
import init, { DesignRenderer } from "design-app-renderer";
import { useAtom, useSetAtom } from "jotai";
import { wasmLoadedAtom } from "../../../store/app";
import { designRendererAtom } from "../../../store/renderer";

const CANVAS_NAME = "preview";

async function initializeRenderer() {
  const renderer = await DesignRenderer.create(CANVAS_NAME);
  await renderer.run();
  return renderer;
}

type Props = {};

const DesignCanvas = (props: Props) => {
  const [wasmLoaded, setWasmLoaded] = useAtom(wasmLoadedAtom);
  const rendererLoaded = useRef(false);
  const setDesignRenderer = useSetAtom(designRendererAtom);

  // Handle initializing WASM + Renderer
  useEffect(() => {
    async function startRenderer() {
      const newRenderer = await initializeRenderer();
      setDesignRenderer(newRenderer);
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
  return (
    <div>
      <canvas id={CANVAS_NAME} width={800} height={600} />
    </div>
  );
};

export default DesignCanvas;
