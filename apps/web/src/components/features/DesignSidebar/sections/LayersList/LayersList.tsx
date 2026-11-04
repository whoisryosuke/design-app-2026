import { useEffect, useState } from "react";
import { Stack } from "../../../../../../styled-system/jsx";
import { designEngineAtom } from "../../../../../store/renderer";
import { useAtomValue } from "jotai";
import { Button } from "../../../../ui/Button/Button";
import { Heading } from "../../../../ui/Typography/Heading";

type Props = {};

const LayersList = (props: Props) => {
  const [layers, setLayers] = useState([]);
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
    renderer.dispatch({
      AddLayer: {
        layer: {
          name: "Test",
          kind: "Rect",
          position: [0, 0],
          width,
          height,
          opacity: 1.0,
          parent: null,
          visible: true,
          items: [],
        },
      },
    });

    console.log("layer created?");
  };

  return (
    <Stack>
      <Heading as="h5">Layers</Heading>
      <Stack></Stack>
      <Button onClick={createDebugLayer}>Create layer</Button>
    </Stack>
  );
};

export default LayersList;
