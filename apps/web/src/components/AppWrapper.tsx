import type { PropsWithChildren } from "react";
// Fonts
import "@fontsource-variable/instrument-sans/wght.css";
import "@fontsource/ibm-plex-mono/400.css";
import "@fontsource/ibm-plex-mono/700.css";
import { Stack } from "../../styled-system/jsx";
import { css, cx } from "../../styled-system/css";
import TopNavigation from "./features/TopNavigation/TopNavigation";

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
        color="gray.12"
        width="100%"
        minHeight="100%"
        maxHeight="100%"
        position="absolute"
        inset={0}
      >
        <TopNavigation />
        {children}
      </Stack>
    </div>
  );
};
