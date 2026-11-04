import React from "react";
import { Stack } from "../../../../styled-system/jsx";
import LayersList from "./sections/LayersList/LayersList";

type Props = {};

const DesignSidebar = (props: Props) => {
  return (
    <Stack width={"300px"}>
      <LayersList />
    </Stack>
  );
};

export default DesignSidebar;
