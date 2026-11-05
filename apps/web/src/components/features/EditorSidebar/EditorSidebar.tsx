import { useAtom, useAtomValue } from "jotai";
import React from "react";
import { layersAtom, selectedLayerAtom } from "../../../store/app";
import LayerEditor from "./sections/LayerEditor/LayerEditor";
import { Stack } from "../../../../styled-system/jsx";

type Props = {};

const EditorSidebar = (props: Props) => {
  const layers = useAtomValue(layersAtom);
  const selectedLayerId = useAtomValue(selectedLayerAtom);
  const selectedLayer = layers.find((layer) => layer.id == selectedLayerId);

  return (
    <Stack bg="gray.4">
      {selectedLayer ? (
        <LayerEditor layer={selectedLayer} />
      ) : (
        "Select a layer first"
      )}
    </Stack>
  );
};

export default EditorSidebar;
