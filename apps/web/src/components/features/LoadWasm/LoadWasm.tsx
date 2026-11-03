import React, { useEffect } from "react";
import init from "design-app-renderer";
import { useAtom } from "jotai";
import { getDesignEngine } from "../../../services/load-wasm";
import { designEngineAtom } from "../../../store/renderer";

type Props = {};

const LoadWasm = (props: Props) => {
  const [wasmLoaded, setDesignEngine] = useAtom(designEngineAtom);

  // Handle initializing WASM + Renderer
  useEffect(() => {
    async function runWasm() {
      const designEngine = await getDesignEngine();
      setDesignEngine(designEngine);
      console.log("✅ Renderer WASM loaded");
    }

    // WASM isn't loaded? Initialize it
    if (!wasmLoaded) runWasm();
  }, [wasmLoaded]);

  return <></>;
};

export default LoadWasm;
