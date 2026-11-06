import type { DesignEvent, UpdateLayer } from "design-app-renderer-types";
import type { DesignEventPayload } from "../utils/types";
import { store } from "../store/store";
import {
  currentMouseCursorAtom,
  layersAtom,
  selectedLayerAtom,
} from "../store/app";

// Update selected layer when clicked on canvas
function handleSelectedLayer(data: DesignEventPayload<"LayerSelected">) {
  store.set(selectedLayerAtom, data.layer_id);
}

// Update layer position when moved on canvas
function handleMovedLayer(data: DesignEventPayload<"LayerMoved">) {
  store.set(layersAtom, (prevState) =>
    prevState.map((layer) =>
      layer.id == data.layer_id ? { ...layer, position: data.position } : layer,
    ),
  );
}

// Update layer when changed in any way (name, position, etc -- see: `UpdateLayer`)
function handleUpdatedLayer(data: DesignEventPayload<"LayerUpdated">) {
  // Rust requires an object with `Option<>` props, which translates to `undefined` when `None`.
  // We create a new object of actual updates, stripping out undefined options.
  const updates = Object.fromEntries(
    Object.entries(data.data).filter(([_, value]) => value !== undefined),
  );
  console.log("updating layer data", data, updates);
  store.set(layersAtom, (prevState) =>
    prevState.map((layer) =>
      layer.id == data.layer_id ? { ...layer, ...updates } : layer,
    ),
  );
}

// Update layer position when moved on canvas
function handleMouseCursorChange(
  data: DesignEventPayload<"MouseCursorChanged">,
) {
  store.set(currentMouseCursorAtom, data.cursor);
}

export function handleBackendEvent(event: DesignEvent) {
  switch (event.type) {
    case "LayerSelected":
      return handleSelectedLayer(event.payload);
    case "LayerMoved":
      return handleMovedLayer(event.payload);
    case "MouseCursorChanged":
      return handleMouseCursorChange(event.payload);
    case "LayerUpdated":
      return handleUpdatedLayer(event.payload);
    default:
  }
}
