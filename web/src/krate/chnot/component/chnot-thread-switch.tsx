import { useState } from "react";
import { ChnotViewType, useChnotStore } from "../store";
import { Button } from "@/common/component/ui/button";
import Icon from "@/common/component/icon";

const Switch = ({}: {}) => {
  const { viewType, setViewType } = useChnotStore((store) => {
    return {
      viewType: store.viewType,
      setViewType: store.setViewType,
    };
  });
  return (
    <Button
      aria-label="Toggle view type"
      className="bg-transparent"
      onClick={() => {
        setViewType(
          viewType === ChnotViewType.Single
            ? ChnotViewType.Thread
            : ChnotViewType.Single,
        );
      }}
    >
      {viewType === ChnotViewType.Single ? <Icon.BrickWall /> : <Icon.Blocks />}
    </Button>
  );
};

export default Switch;
