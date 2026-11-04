import React, { type HTMLProps } from "react";
import { text, type TextVariantProps } from "../../../../styled-system/recipes";
import { css, cx } from "../../../../styled-system/css";

export type TextProps = HTMLProps<HTMLParagraphElement & HTMLSpanElement> &
  TextVariantProps & {
    as?: "p" | "span";
  };

export const Text = ({
  as: ComponentName = "p",
  fontFamily = "body",
  className,
  ...props
}: TextProps) => {
  return (
    <ComponentName className={cx(text({ fontFamily }), className)} {...props} />
  );
};
