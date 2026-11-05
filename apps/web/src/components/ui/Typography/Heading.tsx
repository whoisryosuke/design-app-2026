import React, { type HTMLProps } from "react";
import { text } from "../../../../styled-system/recipes";
import { cva, cx } from "../../../../styled-system/css";

const headingStyles = cva({
  base: {
    fontFamily: "heading",
    fontWeight: 600,
    lineHeight: 1.5,
    color: "gray.12",
  },
  variants: {
    as: {
      h1: {
        fontSize: "7",
      },
      h2: {
        fontSize: "6",
      },
      h3: {
        fontSize: "5",
      },
      h4: {
        fontSize: "4",
      },
      h5: {
        fontSize: "3",
      },
      h6: {
        fontSize: "2",
      },
    },
  },
});

export type HeadingProps = HTMLProps<HTMLHeadingElement> & {
  as?: "h1" | "h2" | "h3" | "h4" | "h5" | "h6";
};

export const Heading = ({ as = "h1", className, ...props }: HeadingProps) => {
  const ComponentName = as;
  return (
    <ComponentName
      className={cx(text(), headingStyles({ as }), className)}
      {...props}
    />
  );
};
