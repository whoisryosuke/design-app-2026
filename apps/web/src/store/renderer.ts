import { atom, useAtomValue } from "jotai";
import { DesignEngine } from "design-app-renderer";

export const designEngineAtom = atom<DesignEngine | null>(null);
