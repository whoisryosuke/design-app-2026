import type { DesignCommand, DraftLayer } from "design-app-renderer-types";

export function generateMockLayer() {
  const layer: DraftLayer = {
    name: null,
    kind: {
      type: "Rect",
    },
    position: [0, 0],
    width: 420,
    height: 420,
    visible: false,
    opacity: 1,
    color: {
      r: 0,
      g: 0,
      b: 255,
      a: 255,
    },
    parent: null,
    items: [],
  };
  return layer;
}

export function generateDesignCommandAddLayer() {
  const command: DesignCommand = {
    type: "AddLayer",
    payload: {
      layer: generateMockLayer(),
    },
  };
  return command;
}
