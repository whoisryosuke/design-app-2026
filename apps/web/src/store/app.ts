import type { DraftLayer } from "design-app-renderer-types";
import { atom } from "jotai";

// The Layer ID provided from Rust backend.
// It's a SlotMap `DefaultKey` converted to JS.
export type LayerId = bigint;
export type UILayer = DraftLayer & {
  id: LayerId;
};

export const selectedLayerAtom = atom<LayerId | null>(null);
export const layersAtom = atom<UILayer[]>([]);
