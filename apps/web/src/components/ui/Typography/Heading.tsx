import React, { type HTMLProps } from "react";
import { text } from "../../../../styled-system/recipes";
import { cva, cx } from "../../../../styled-system/css";

const headingStyles = cva({
  base: {
    fontFamily: "heading",
    fontWeight: "bold",
    lineHeight: 1.1,
    color: "gray.12",
  },
  variants: {
    as: {
      h1: {
        fontSize: "textSize.h1",
      },
      h2: {
        fontSize: "textSize.h2",
      },
      h3: {
        fontSize: "textSize.h3",
      },
      h4: {
        fontSize: "textSize.h4",
      },
      h5: {
        fontSize: "textSize.h5",
      },
      h6: {
        fontSize: "textSize.h5",
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
