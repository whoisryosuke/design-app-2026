import { useEffect, useState } from "react";
import { Box, Stack } from "../../../../../../styled-system/jsx";
import { designEngineAtom } from "../../../../../store/renderer";
import { useAtom, useAtomValue } from "jotai";
import { Button } from "../../../../ui/Button/Button";
import { Heading } from "../../../../ui/Typography/Heading";
import type { DesignCommand, DraftLayer } from "design-app-renderer-types";
import { Text } from "../../../../ui/Typography/Text";
import {
  layersAtom,
  selectedLayerAtom,
  type LayerId,
  type UILayer,
} from "../../../../../store/app";
import LayerListItem from "./LayerListItem";

type Props = {};

const LayersList = (props: Props) => {
  const [layers, setLayers] = useAtom(layersAtom);
  const [selectedLayer, setSelectedLayer] = useAtom(selectedLayerAtom);
  const renderer = useAtomValue(designEngineAtom);

  useEffect(() => {
    if (!renderer) return;
    const newLayers = renderer.dispatch({
      type: "FetchLayersByParent",
      payload: {
        parent_id: null,
      },
    });

    console.log("newLayers", newLayers);
  }, [renderer]);

  const createDebugLayer = () => {
    if (!renderer) return;

    const width = Math.round(Math.random() * 420);
    const height = Math.round(Math.random() * 420);
    const x = Math.round(Math.random() * 420);
    const y = Math.round(Math.random() * 420);
    const draftLayer: DraftLayer = {
      name: "Test",
      kind: {
        type: "Rect",
      },
      position: [x, y],
      width,
      height,
      opacity: 1.0,
      color: {
        r: Math.random(),
        g: Math.random(),
        b: Math.random(),
        a: 1.0,
      },
      parent: null,
      visible: true,
      items: [],
    };
    const id = renderer.dispatch({
      type: "AddLayer",
      payload: {
        layer: draftLayer,
      },
    });

    console.log("layer created?", id, draftLayer);

    // @TODO: re-order using fractional index
    setLayers((prev) => [...prev, { ...draftLayer, id }]);
  };

  const handleSelectLayer = (id: UILayer["id"]) => {
    if (!renderer) return;
    console.log("layer selected", id);
    // Select in backend
    renderer.dispatch({
      type: "SelectLayer",
      payload: {
        layer_id: id,
      },
    } as DesignCommand);

    // Select in frontend
    setSelectedLayer(id);
  };

  return (
    <Stack>
      <Stack borderBottomWidth="1px" borderColor="gray.6" borderStyle="solid">
        <Box px="3" py="2">
          <Heading as="h5">Pages</Heading>
        </Box>
      </Stack>
      <Box
        px="3"
        py="2"
        borderBottomWidth="1px"
        borderColor="gray.6"
        borderStyle="solid"
      >
        <Heading as="h5">Layers</Heading>
      </Box>
      <Stack gap="0">
        {layers.map((layer) => (
          <LayerListItem
            key={layer.id}
            layer={layer}
            selected={layer.id == selectedLayer}
            handleSelectLayer={handleSelectLayer}
          />
        ))}
      </Stack>
      <Button onClick={createDebugLayer}>Create layer</Button>
    </Stack>
  );
};

export default LayersList;
