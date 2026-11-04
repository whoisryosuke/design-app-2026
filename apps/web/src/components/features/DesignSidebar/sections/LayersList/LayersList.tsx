import { useEffect, useState } from "react";
import { Stack } from "../../../../../../styled-system/jsx";
import { designEngineAtom } from "../../../../../store/renderer";
import { useAtomValue } from "jotai";
import { Button } from "../../../../ui/Button/Button";
import { Heading } from "../../../../ui/Typography/Heading";
import type { DesignCommand, DraftLayer } from "design-app-renderer-types";
import { Text } from "../../../../ui/Typography/Text";

type UILayer = DraftLayer & {
  order: bigint;
};

type Props = {};

const LayersList = (props: Props) => {
  const [layers, setLayers] = useState<UILayer[]>([]);
  const renderer = useAtomValue(designEngineAtom);

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
    const order = renderer.dispatch({
      AddLayer: {
        layer: draftLayer,
      },
    });

    console.log("layer created?", order, draftLayer);

    // @TODO: re-order using fractional index (aka `order`)
    setLayers((prev) => [...prev, { ...draftLayer, order }]);
  };

  const handleSelectLayer = (order: UILayer["order"]) => {
    if (!renderer) return;
    renderer.dispatch({
      SelectLayer: {
        layer_id: order,
      },
    } as DesignCommand);
  };

  return (
    <Stack>
      <Heading as="h5">Layers</Heading>
      <Stack>
        {layers.map((layer, index) => (
          <Text
            id={`${layer.name}${index}`}
            onClick={() => handleSelectLayer(layer.order)}
          >
            {layer.name} ({layer.kind.toString()})
          </Text>
        ))}
      </Stack>
      <Button onClick={createDebugLayer}>Create layer</Button>
    </Stack>
  );
};

export default LayersList;
