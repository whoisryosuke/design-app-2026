import type { PropsWithChildren } from "react";
// Fonts
import "@fontsource-variable/instrument-sans/wght.css";
import "@fontsource/ibm-plex-mono/400.css";
import "@fontsource/ibm-plex-mono/700.css";
import { Stack } from "../../styled-system/jsx/stack";

export type AppWrapperProps = {
  theme?: "light" | "dark";
  // uiScale?: UIScaleThemes;
};

export const AppWrapper = ({
  theme,
  children,
}: PropsWithChildren<AppWrapperProps>) => {
  return (
    <body className={theme == "dark" ? "dark" : ""}>
      <Stack
        backgroundColor="gray.1"
        width="100%"
        minHeight="100%"
        maxHeight="100%"
      >
        {children}
      </Stack>
    </body>
  );
};
