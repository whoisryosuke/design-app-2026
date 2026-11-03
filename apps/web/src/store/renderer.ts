import { atom, useAtomValue } from "jotai";
import { store } from "./store";
import { DesignEngine } from "design-app-renderer";
import type { DesignCommand } from "design-app-renderer-types";

export const designEngineAtom = atom<DesignEngine | null>(null);
