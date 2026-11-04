import type { ButtonHTMLAttributes, DetailedHTMLProps } from "react";
import {
  button,
  type ButtonVariantProps,
} from "../../../../styled-system/recipes";
import { css, cx } from "../../../../styled-system/css";
import type { ColorPalette } from "../../../../styled-system/types";

export type ButtonProps = DetailedHTMLProps<
  ButtonHTMLAttributes<HTMLButtonElement>,
  HTMLButtonElement
> &
  ButtonVariantProps & {
    color?: ColorPalette;
  };

export const Button = ({
  appearance = "solid",
  color = "blue",
  className,
  ...props
}: ButtonProps) => {
  return (
    <button
      className={cx(
        css({ colorPalette: color }),
        button({ appearance }),
        className,
      )}
      {...props}
    />
  );
};
