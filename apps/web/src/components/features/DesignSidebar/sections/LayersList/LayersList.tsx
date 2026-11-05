import { useEffect, useState } from "react";
import { Stack } from "../../../../../../styled-system/jsx";
import { designEngineAtom } from "../../../../../store/renderer";
import { useAtom, useAtomValue } from "jotai";
import { Button } from "../../../../ui/Button/Button";
import { Heading } from "../../../../ui/Typography/Heading";
import type { DesignCommand, DraftLayer } from "design-app-renderer-types";
import { Text } from "../../../../ui/Typography/Text";
import {
  selectedLayerAtom,
  type LayerId,
  type UILayer,
} from "../../../../../store/app";
import LayerListItem from "./LayerListItem";

type Props = {};

const LayersList = (props: Props) => {
  const [layers, setLayers] = useState<UILayer[]>([]);
  const renderer = useAtomValue(designEngineAtom);
  const [selectedLayer, setSelectedLayer] = useAtom(selectedLayerAtom);

  useEffect(() => {
    if (!renderer) return;
    const newLayers = renderer.dispatch({
      FetchLayersByParent: {
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
      kind: "Rect",
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
      AddLayer: {
        layer: draftLayer,
      },
    });

    console.log("layer created?", id, draftLayer);

    // @TODO: re-order using fractional index
    setLayers((prev) => [...prev, { ...draftLayer, id }]);
  };

  const handleSelectLayer = (id: UILayer["id"]) => {
    if (!renderer) return;
    // Select in backend
    renderer.dispatch({
      SelectLayer: {
        layer_id: id,
      },
    } as DesignCommand);

    // Select in frontend
    setSelectedLayer(id);
  };

  return (
    <Stack>
      <Heading as="h5">Layers</Heading>
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
