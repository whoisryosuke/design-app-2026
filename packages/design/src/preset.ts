import { breakpoints } from "./tokens/breakpoints";
import { containerSizes } from "./tokens/containers";
import { keyframes } from "./tokens/keyframes";
import { tokens } from "./tokens/tokens";
import { fontsSemantic, textStyles } from "./tokens/typography";
import { semanticColors } from "./tokens/colors";
import textRecipe from "./recipes/text.recipe";
import buttonRecipe from "./recipes/button.recipe";
import { Preset } from "@pandacss/dev";

const definePreset = <T extends Preset>(config: T) => config;

export const tuningWavesPreset = definePreset({
  name: "design-app-preset",
  theme: {
    keyframes,
    breakpoints,
    tokens,
    textStyles,
    containerSizes,
    recipes: {
      text: textRecipe,
      button: buttonRecipe,
    },
    semanticTokens: {
      colors: semanticColors,
      fonts: fontsSemantic,
    },
  },

  conditions: {
    extend: {
      hoverEnabled:
        "&:is(:hover, [data-hover]):not(:disabled, [data-disabled])",
      activeEnabled:
        "&:is(:active, [data-active]):not(:disabled, [data-disabled])",
    },
  },
});

export default tuningWavesPreset;
