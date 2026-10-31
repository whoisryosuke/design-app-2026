import { defineConfig } from "@pandacss/dev";
import oatMilkPreset from "./src/preset";

export default defineConfig({
  presets: ["@pandacss/preset-base", oatMilkPreset],

  // Whether to use css reset
  preflight: true,

  // Where to look for your css declarations
  include: ["./src/**/*.{js,jsx,ts,tsx}", "./pages/**/*.{js,jsx,ts,tsx}"],

  // Files to exclude
  exclude: [],

  // Useful for theme customization
  theme: {
    extend: {},
  },

  // The output directory for your css system
  outdir: "dist",

  // Generates JSX utilities for React
  jsxFramework: "react",
});
