import { Squirrel, Target } from "lucide-react";
import { Button } from "@/common/component/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/common/component/ui/dropdown-menu";
import { ChnotKind } from "../../po";
import { useChnotStore } from "../../store";
import { ChnotKindIcon } from "../kind-icon";

export const ChnotKindSelect = () => {
  const { kinds, setChnotKinds } = useChnotStore((s) => {
    return {
      kinds: s.kinds,
      setChnotKinds: s.setChnotKinds,
    };
  });

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button aria-label="Select knowledge space" className="bg-transparent">
          {kinds && kinds.length > 0 ? (
            kinds.map((kind) => {
              return <ChnotKindIcon kind={kind} key={kind} />;
            })
          ) : (
            <Squirrel />
          )}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-56">
        <DropdownMenuLabel>Chnot Kind</DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem
            key={"all"}
            onClick={() => setChnotKinds((_) => [])}
          >
            <Squirrel />
            <span>all</span>
          </DropdownMenuItem>
          {Object.values(ChnotKind).map((e) => (
            <DropdownMenuItem key={e}>
              <div className="flex items-center justify-between w-full">
                <button
                  type="button"
                  className="flex items-center flex-1 gap-2 cursor-pointer"
                  onClick={() =>
                    setChnotKinds((kinds) => {
                      return [e, ...(kinds ?? [])];
                    })
                  }
                  onKeyDown={(event) =>
                    (event.key === "Enter" || event.key === " ") &&
                    setChnotKinds((kinds) => {
                      return [e, ...(kinds ?? [])];
                    })
                  }
                >
                  <ChnotKindIcon kind={e} />
                  <span className="truncate">{e}</span>
                </button>

                <Button
                  variant="ghost"
                  className="hover:bg-accent h-6"
                  onClick={(event) => {
                    event.stopPropagation();
                    setChnotKinds(() => [e]);
                  }}
                  aria-label={`Add ${e} to mkspace`}
                >
                  <Target />
                </Button>
              </div>
            </DropdownMenuItem>
          ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
