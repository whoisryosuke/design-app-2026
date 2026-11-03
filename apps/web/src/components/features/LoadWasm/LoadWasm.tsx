import React, { useEffect } from "react";
import init from "design-app-renderer";
import { useAtom } from "jotai";
import { wasmLoadedAtom } from "../../../store/app";

type Props = {};

const LoadWasm = (props: Props) => {
  const [wasmLoaded, setWasmLoaded] = useAtom(wasmLoadedAtom);

  // Handle initializing WASM + Renderer
  useEffect(() => {
    async function runWasm() {
      await init();
      setWasmLoaded(true);
      console.log("✅ Renderer WASM loaded");
    }

    // WASM isn't loaded? Initialize it
    if (!wasmLoaded) runWasm();
  }, [wasmLoaded]);

  return <></>;
};

export default LoadWasm;
