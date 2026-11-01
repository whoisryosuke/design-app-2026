import React from "react";
import { Stack } from "../../../../styled-system/jsx";
import Layers from "./sections/Layers/Layers";

type Props = {};

const DesignSidebar = (props: Props) => {
  return (
    <Stack>
      <Layers />
    </Stack>
  );
};

export default DesignSidebar;
