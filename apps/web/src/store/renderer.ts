import { atom } from "jotai";
import { store } from "./store";
import { DesignRenderer } from "design-app-renderer";
import type { DesignCommand } from "design-app-renderer-types";

export const designRendererAtom = atom<DesignRenderer | null>(null);

export const dispatchCommandToRenderer = (command: DesignCommand) => {
  const renderer = store.get(designRendererAtom);
  if (!renderer) return;

  renderer.dispatch(command);
};
