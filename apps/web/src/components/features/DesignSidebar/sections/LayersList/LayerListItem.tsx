import React from "react";
import type { LayerId, UILayer } from "../../../../../store/app";
import { cva, cx } from "../../../../../../styled-system/css";
import { text } from "design-app-design-system/recipes";

const itemStyle = cva({
  base: {
    display: "flex",
    bg: {
      base: "transparent",
      _hover: "gray.alpha-3",
    },
    px: 3,
    py: 2,

    _motionSafe: {
      transitionProperty: "background-color, color",
      transitionTimingFunction: "ease-in",
      transitionDuration: "fastest",
    },
  },
  variants: {
    selected: {
      true: {
        bg: {
          base: "blue.alpha-3",
          _hover: "blue.alpha-4",
        },
      },
    },
  },
});

type Props = {
  layer: UILayer;
  handleSelectLayer: (id: LayerId) => void;
  selected: boolean;
};

const LayerListItem = ({
  layer,
  selected,
  handleSelectLayer,
  ...props
}: Props) => {
  const itemClass = itemStyle({ selected });

  const handleClick = () => handleSelectLayer(layer.id);
  return (
    <button className={cx(text(), itemClass)} onClick={handleClick} {...props}>
      {layer.name} ({layer.kind.toString()})
    </button>
  );
};

export default LayerListItem;
