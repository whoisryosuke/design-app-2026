import React from "react";
import type { UILayer } from "../../../../../store/app";
import { Heading } from "../../../../ui/Typography/Heading";
import { Text } from "../../../../ui/Typography/Text";
import { Stack } from "../../../../../../styled-system/jsx";
import NumberInput from "../../../../ui/Input/NumberInput/NumberInput";
import { css } from "../../../../../../styled-system/css";

const headingStyle = css({
  color: "gray.10",
});

type Props = {
  layer: UILayer;
};

const LayerEditor = ({ layer, ...props }: Props) => {
  return (
    <Stack minWidth="300px" p={3}>
      <Heading as="h5">{layer.kind.toString()}</Heading>
      <Heading as="h6" className={headingStyle}>
        Position
      </Heading>
      <Stack flexDirection="row" alignItems="end">
        <NumberInput
          name="position-x"
          label="Position"
          icon={<Text>X</Text>}
          value={layer.position[0]}
          onChange={() => {}}
        />
        <NumberInput
          name="position-y"
          icon={<Text>Y</Text>}
          value={layer.position[1]}
          onChange={() => {}}
        />
      </Stack>
      <Heading as="h6" className={headingStyle}>
        Layout
      </Heading>
      <Stack flexDirection="row" alignItems="end">
        <NumberInput
          name="dimension-width"
          label="Dimensions"
          icon={<Text>W</Text>}
          value={layer.width}
          onChange={() => {}}
        />
        <NumberInput
          name="dimension-height"
          icon={<Text>H</Text>}
          value={layer.height}
          onChange={() => {}}
        />
      </Stack>
    </Stack>
  );
};

export default LayerEditor;
