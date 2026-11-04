import React, { type HTMLProps } from "react";
import { text, type TextVariantProps } from "../../../../styled-system/recipes";
import { css, cx, type RecipeVariant } from "../../../../styled-system/css";

const textStyles = css({
  color: "gray.12",
  marginBottom: 2,
  fontSize: "textSize.p",
});

export type ParagraphProps = HTMLProps<HTMLParagraphElement> & {
  as?: "p";
} & TextVariantProps & {};

export const Paragraph = ({
  className,
  fontFamily = "body",
  ...props
}: ParagraphProps) => {
  return (
    <p className={cx(text({ fontFamily }), textStyles, className)} {...props} />
  );
};
