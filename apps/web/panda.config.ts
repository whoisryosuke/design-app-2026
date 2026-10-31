import { defineConfig } from "@pandacss/dev";

export default defineConfig({
  designSystem: "design-app-design-system",
  include: ["./src/**/*.{js,jsx,ts,tsx}", "./pages/**/*.{js,jsx,ts,tsx}"],
  exclude: [],
  theme: {
    extend: {},
  },
  outdir: "styled-system",

  // Generates JSX utilities for React
  jsxFramework: "react",

  staticCss: {
    css: [
      {
        properties: {
          colorPalette: [
            "tomato",
            "red",
            "crimson",
            "pink",
            "plum",
            "purple",
            "violet",
            "indigo",
            "blue",
            "cyan",
            "teal",
            "green",
            "grass",
            "orange",
            "brown",
            "sky",
            "mint",
            "lime",
            "yellow",
            "amber",
            "gray",
            "mauve",
            "slate",
            "sage",
            "olive",
            "sand",
            "bronze",
            "gold",
            "ruby",
            "iris",
            "jade",
          ],
        },
      },
    ],
  },
});
