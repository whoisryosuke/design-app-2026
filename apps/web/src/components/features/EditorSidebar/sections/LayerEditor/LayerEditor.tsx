import React from "react";
import type { UILayer } from "../../../../../store/app";
import { Heading } from "../../../../ui/Typography/Heading";
import { Text } from "../../../../ui/Typography/Text";
import { Stack } from "../../../../../../styled-system/jsx";
import NumberInput, {
  type InputValue,
} from "../../../../ui/Input/NumberInput/NumberInput";
import { css } from "../../../../../../styled-system/css";
import { useAtomValue } from "jotai";
import { designEngineAtom } from "../../../../../store/renderer";
import type { DesignCommand, UpdateLayer } from "design-app-renderer-types";

const headingStyle = css({
  color: "gray.10",
});

type Props = {
  layer: UILayer;
};

const LayerEditor = ({ layer, ...props }: Props) => {
  const engine = useAtomValue(designEngineAtom);
  const handleNumberChange =
    (prop: keyof UpdateLayer) => (value: InputValue) => {
      const parsed = parseFloat(`${value}`);

      // Save to backend
      // @ts-expect-error - I use `keyof` above...
      engine?.dispatch({
        type: "UpdateLayer",
        payload: {
          layer_id: layer.id,
          data: {
            [prop]: parsed,
          },
        },
      } as DesignCommand);

      // Send to input to update internal state
      return parsed;
    };

  return (
    <Stack minWidth="300px" p={3}>
      <Heading as="h5">{layer.kind.type}</Heading>
      <Heading as="h6" className={headingStyle}>
        Position
      </Heading>
      <Stack flexDirection="row" alignItems="end">
        <NumberInput
          name="position-x"
          label="Position"
          icon={<Text>X</Text>}
          defaultValue={layer.position[0]}
          onSubmit={handleNumberChange("position_x")}
        />
        <NumberInput
          name="position-y"
          icon={<Text>Y</Text>}
          defaultValue={layer.position[1]}
          onSubmit={handleNumberChange("position_y")}
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
          defaultValue={layer.width}
          onSubmit={handleNumberChange("width")}
        />
        <NumberInput
          name="dimension-height"
          icon={<Text>H</Text>}
          defaultValue={layer.height}
          onSubmit={handleNumberChange("height")}
        />
      </Stack>
    </Stack>
  );
};

export default LayerEditor;
