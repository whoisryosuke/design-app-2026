import { defineRecipe } from "@pandacss/dev";

const buttonRecipe = defineRecipe({
  className: "button",
  base: {
    border: 0,
    fontSize: "textSize.p",
    fontWeight: "normal",
    lineHeight: 1,

    borderRadius: 4,
    margin: 0,
    py: {
      base: 2,
      md: 3,
    },
    px: {
      base: 4,
      md: 8,
    },

    _disabled: {
      cursor: "default",
    },

    _motionSafe: {
      transitionProperty: "background-color, color",
      transitionTimingFunction: "ease-in-out",
      transitionDuration: "slow",
    },
  },
  variants: {
    appearance: {
      solid: {
        color: "colorPalette.1",
        backgroundColor: {
          base: "colorPalette.9",
          _hoverEnabled: "colorPalette.10",
          _activeEnabled: "colorPalette.11",
          _disabled: "gray.6",
        },
      },
      ghost: {
        color: "gray.1",
      },
      outline: {
        color: "colorPalette.1",
        borderWidth: "1.5px",
        borderStyle: "solid",
        borderColor: {
          base: "colorPalette.9",
          _hoverEnabled: "colorPalette.10",
          _activeEnabled: "colorPalette.11",
          _disabled: "gray.6",
        },
      },
    },
  },
});

export default buttonRecipe;
