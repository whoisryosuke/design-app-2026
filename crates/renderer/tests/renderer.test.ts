import { beforeAll, describe, expect, test } from "vitest";
import init, { DesignEngine } from "../pkg/renderer";
import type { DesignCommand } from "design-app-renderer-types";
import { generateDesignCommandAddLayer } from "./helpers/factories";

beforeAll(async () => {
  await init(); // fetches and instantiates the .wasm once for this file
});

describe("DesignEngine", () => {
  test("stores new layer in backend and returns layer ID as bigint", () => {
    const engine = new DesignEngine();

    try {
      const command = generateDesignCommandAddLayer();

      const id = engine.dispatch(command);

      expect(id).exist;
      expect(id).toBeTypeOf("bigint");
    } finally {
      engine.free();
    }
  });

  test("stores new layer in backend and updates layer position", () => {
    const engine = new DesignEngine();

    try {
      const command_add_layer = generateDesignCommandAddLayer();
      const layer_id = engine.dispatch(command_add_layer);

      const command_move_layer: DesignCommand = {
        type: "UpdateLayer",
        payload: {
          layer_id,
          data: {
            position_x: 420,
            name: null,
            kind: null,
            position_y: null,
            width: null,
            height: null,
            visible: null,
            opacity: null,
            color: null,
            parent: null,
          },
        },
      };
      const result = engine.dispatch(command_move_layer);

      expect(result).toBe(true);
    } finally {
      engine.free();
    }
  });

  test("stores new layer in backend and fetches layers in backend to confirm same layer ID", () => {
    const engine = new DesignEngine();

    try {
      const command_add_layer = generateDesignCommandAddLayer();
      const id = engine.dispatch(command_add_layer);

      const command_fetch_layers: DesignCommand = {
        type: "FetchLayersByParent",
        payload: {
          parent_id: null,
        },
      };
      const result = engine.dispatch(command_fetch_layers);

      console.log("layers", result);

      expect(result).toHaveLength(1);
      expect(result[0].id).toBe(id);
    } finally {
      engine.free();
    }
  });
});
