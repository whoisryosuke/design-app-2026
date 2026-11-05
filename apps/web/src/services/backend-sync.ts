import type { DesignEvent } from "design-app-renderer-types";
import type { DesignEventPayload } from "../utils/types";
import { store } from "../store/store";
import { selectedLayerAtom } from "../store/app";

function handleSelectedLayer(data: DesignEventPayload<"LayerSelected">) {
  store.set(selectedLayerAtom, data.layer_id);
}

export function handleBackendEvent(event: DesignEvent) {
  switch (event.type) {
    case "LayerSelected":
      return handleSelectedLayer(event.payload);
    default:
  }
}
