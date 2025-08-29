import { ChnotKind } from "@/krate/chnot/po";
import { Toggle } from "@radix-ui/react-toggle";
import { ChnotKindIcon } from "../../chnot-kind-icon";

const Tier = ({
  setKind,
  kind,
  hidden,
}: {
  setKind: (kind?: ChnotKind) => void;
  kind?: ChnotKind;
  hidden: boolean;
}) => {
  return (
    hidden || (
      <div>
        {Object.values(ChnotKind).map((e) => (
          <Toggle
            key={e}
            onPressedChange={(pressed) => {
              setKind(pressed ? e : undefined);
            }}
            pressed={kind === e}
          >
            <ChnotKindIcon className="w-4 h-4" kind={e} />
          </Toggle>
        ))}
      </div>
    )
  );
};

export default Tier;
