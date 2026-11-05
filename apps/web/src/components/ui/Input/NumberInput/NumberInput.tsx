import React, { type ChangeEventHandler, type ReactElement } from "react";
import { Stack } from "../../../../../styled-system/jsx";
import { cx, sva } from "../../../../../styled-system/css";

const inputStyle = sva({
  slots: ["container", "content", "label", "input"],
  base: {
    container: {
      fontSize: 1,
      flex: 1,
    },
    content: {
      borderRadius: 2,
      bg: {
        base: "gray.6",
        _hover: "gray.7",
      },
      px: 2,
      py: 2,
    },
    label: {
      fontWeight: 600,
      color: "gray.8",
    },
    input: {
      fontFamily: "mono",
    },
  },
});

type Props = {
  label?: string;
  name: string;
  icon?: ReactElement;
  value: number;
  onChange?: ChangeEventHandler<HTMLInputElement, HTMLInputElement>;
};

const NumberInput = ({
  label,
  icon,
  name,
  value,
  onChange,
  ...props
}: Props) => {
  const classes = inputStyle();
  return (
    <Stack className={classes.container} {...props}>
      {label && (
        <label htmlFor={name} className={classes.label}>
          {label}
        </label>
      )}
      <Stack className={cx(classes.content)} flexDirection="row">
        {icon}
        <input
          className={classes.input}
          name={name}
          value={value}
          onChange={onChange}
        />
      </Stack>
    </Stack>
  );
};

export default NumberInput;
