import { atom, useAtomValue } from "jotai";
import { store } from "./store";
import { DesignEngine } from "design-app-renderer";
import type { DesignCommand } from "design-app-renderer-types";

export const designEngineAtom = atom<DesignEngine | null>(null);

export const dispatchCommandToRenderer = (command: DesignCommand) => {
  const renderer = store.get(designEngineAtom);
  if (!renderer) {
    console.error("Can't dispatch command, Renderer not initialized.");
    return;
  }

  renderer.dispatch(command);
};
