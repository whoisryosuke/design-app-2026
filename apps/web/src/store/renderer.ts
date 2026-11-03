import { atom, useAtomValue } from "jotai";
import { store } from "./store";
import { DesignRenderer } from "design-app-renderer";
import type { DesignCommand } from "design-app-renderer-types";
import { wasmLoadedAtom } from "./app";
import { useEffect } from "react";

export const designRendererAtom = atom<DesignRenderer | null>(null);

export const dispatchCommandToRenderer = (command: DesignCommand) => {
  const renderer = store.get(designRendererAtom);
  if (!renderer) {
    console.error("Can't dispatch command, Renderer not initialized.");
    return;
  }

  renderer.dispatch(command);
};
