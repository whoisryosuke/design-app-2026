import { useEffect, useState } from "react";
import { Stack } from "../../../../../../styled-system/jsx";
import { designEngineAtom } from "../../../../../store/renderer";
import { useAtomValue } from "jotai";

type Props = {};

const Layers = (props: Props) => {
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

  return (
    <Stack>
      <h3>Layers</h3>
      <Stack></Stack>
    </Stack>
  );
};

export default Layers;
