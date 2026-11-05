import React, {
  useRef,
  useState,
  type ChangeEventHandler,
  type KeyboardEventHandler,
  type ReactElement,
} from "react";
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

export type InputValue = string | number;

type Props = {
  label?: string;
  name: string;
  icon?: ReactElement;
  defaultValue: InputValue;
  onSubmit: (value: InputValue) => InputValue;
};

const NumberInput = ({
  label,
  icon,
  name,
  defaultValue,
  onSubmit,
  ...props
}: Props) => {
  const [value, setValue] = useState(defaultValue);
  const inputEl = useRef<HTMLInputElement | null>(null);
  const isEditing = useRef(false);
  const classes = inputStyle();

  const handleKeyDown: KeyboardEventHandler = (e) => {
    if (e.key == "Escape") {
      inputEl.current?.blur();
      return;
    }
    if (e.key == "Enter") {
      setValue(onSubmit(value));
      isEditing.current = false;
      inputEl.current?.blur();
      return;
    }
  };

  const onChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    setValue(e.currentTarget.value);
  };

  const handleFocus = () => {
    isEditing.current = true;
  };

  const handleBlur = () => {
    // Exiting without saving? Revert.
    if (isEditing.current) {
      setValue(defaultValue);
    }
    // Revert editing state as a fail-safe
    isEditing.current = false;
  };

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
          ref={inputEl}
          className={classes.input}
          name={name}
          value={value}
          onChange={onChange}
          onKeyDown={handleKeyDown}
          onFocus={handleFocus}
          onBlur={handleBlur}
        />
      </Stack>
    </Stack>
  );
};

export default NumberInput;
