import type { PropsWithChildren } from "react";
// Fonts
import "@fontsource-variable/instrument-sans/wght.css";
import "@fontsource/ibm-plex-mono/400.css";
import "@fontsource/ibm-plex-mono/700.css";
import { Stack } from "../../styled-system/jsx";
import { css, cx } from "../../styled-system/css";

export type AppWrapperProps = {
  theme?: "light" | "dark";
  className?: string;
  // uiScale?: UIScaleThemes;
};

export const AppWrapper = ({
  theme,
  children,
  className,
}: PropsWithChildren<AppWrapperProps>) => {
  return (
    <div className={cx(theme == "dark" ? "dark" : "", className)}>
      <Stack
        backgroundColor="gray.1"
        width="100%"
        minHeight="100%"
        maxHeight="100%"
      >
        {children}
      </Stack>
    </div>
  );
};
