import { useEffect, useState } from "react";
import { Stack } from "../../../../../../styled-system/jsx";
import {
  designRendererAtom,
  dispatchCommandToRenderer,
} from "../../../../../store/renderer";
import { useAtomValue } from "jotai";
import { wasmLoadedAtom } from "../../../../../store/app";

type Props = {};

const Layers = (props: Props) => {
  const [layers, setLayers] = useState([]);
  const renderer = useAtomValue(designRendererAtom);

  useEffect(() => {
    if (!renderer) return;
    const newLayers = dispatchCommandToRenderer({
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
