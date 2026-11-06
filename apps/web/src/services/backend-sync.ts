import type { DesignEvent } from "design-app-renderer-types";
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
    default:
  }
}
