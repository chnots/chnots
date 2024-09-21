import { useState } from "react";
import Icon from "./icon";
import { useNamespaceStore } from "@/store/namespace";
import { Namespace } from "@/model";
import clsx from "clsx";

export const NamespaceIcon = ({
  name,
  className,
}: {
  name: string;
  className?: string;
}) => {
  if (name === "public") {
    return <Icon.Globe2 className={clsx(className, "w-6 h-auto shrink-0")} />;
  } else if (name === "work") {
    return (
      <Icon.BriefcaseBusiness
        className={clsx(className, "w-6 h-auto shrink-0")}
      />
    );
  } else {
    return <Icon.Notebook className={clsx(className, "w-6 h-auto shrink-0")} />;
  }
};

export const NamespaceSelect = () => {
  const namespaceStore = useNamespaceStore();
  const [expandState, setExpandState] = useState(false);

  const clickToExpand = () => {
    setExpandState(true);
    namespaceStore.fetchNamespaces();
  };

  const clickToSelect = (ns: Namespace) => {
    namespaceStore.changeNamespace(ns.name);
    setExpandState(false);
  };

  return (
    <div className="flex flex-col border rounded-2xl border-gray-200 p-2">
      {expandState ? (
        <div className=" space-y-4 bg-white">
          {namespaceStore.namespaceMapByName.values().map((ns) => {
            return (
              <div
                onClick={() => clickToSelect(ns)}
                className={clsx(
                  "hover:cursor-pointer",
                  namespaceStore.current.name === ns.name
                    ? "text-neutral-950"
                    : "text-neutral-400"
                )}
              >
                <NamespaceIcon name={ns.name} />
              </div>
            );
          })}
        </div>
      ) : (
        <div onClick={clickToExpand} className="hover:cursor-pointer">
          <NamespaceIcon name={namespaceStore.current.name} />
        </div>
      )}
    </div>
  );
};
