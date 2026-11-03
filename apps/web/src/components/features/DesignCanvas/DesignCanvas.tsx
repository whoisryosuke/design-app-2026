import { useEffect, useRef } from "react";
import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { designEngineAtom } from "../../../store/renderer";

const CANVAS_NAME = "preview";

type Props = {};

const DesignCanvas = (props: Props) => {
  const designEngine = useAtomValue(designEngineAtom);
  const rendererLoaded = useRef(false);

  // Handle initializing WASM + Renderer
  useEffect(() => {
    async function startRenderer() {
      const result = await designEngine?.create_render(CANVAS_NAME);
      console.log("Renderer created with canvas", CANVAS_NAME, result);
      rendererLoaded.current = true;
    }
    // Loaded and haven't run renderer yet? Initialize it and run render loop
    if (designEngine && !rendererLoaded.current) startRenderer();
  }, [designEngine]);
  return (
    <div>
      <canvas id={CANVAS_NAME} width={800} height={600} />
    </div>
  );
};

export default DesignCanvas;
