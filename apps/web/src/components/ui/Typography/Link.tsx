import React, { type HTMLProps } from "react";
import { text, type TextVariantProps } from "../../../../styled-system/recipes";
import {
  css,
  cva,
  cx,
  type RecipeVariant,
} from "../../../../styled-system/css";

const textStyles = cva({
  base: {
    color: {
      base: "gray.11",
      _hover: "teal.9",
    },
    _motionSafe: {
      transitionProperty: "color",
      transitionTimingFunction: "ease-in",
      transitionDuration: "slow",
    },
  },
  variants: {
    underline: {
      false: {
        textDecoration: "none",
      },
      true: {
        textDecoration: "underline",
      },
    },
  },
});

export type LinkProps = HTMLProps<HTMLAnchorElement> &
  Partial<RecipeVariant<typeof textStyles>> &
  TextVariantProps & {};

export const Link = ({
  underline = false,
  fontFamily = "body",
  className,
  ...props
}: LinkProps) => {
  return (
    <a
      className={cx(text({ fontFamily }), textStyles({ underline }), className)}
      {...props}
    />
  );
};
