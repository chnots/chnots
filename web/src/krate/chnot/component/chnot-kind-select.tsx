import Icon from "@/common/component/icon";
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
import { useChnotStore } from "../store";
import { ChnotKindIcon } from "./chnot-kind-icon";
import { ChnotKind } from "../po";

export const ChnotKindSelect = () => {
  const { kinds, setChnotKinds } = useChnotStore();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button aria-label="Select knowledge space" className="bg-transparent">
          {kinds && kinds.length > 0 ? (
            kinds.map((kind) => {
              return <ChnotKindIcon kind={kind} key={kind} />;
            })
          ) : (
            <Icon.Squirrel />
          )}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-56">
        <DropdownMenuLabel>KSpace</DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem
            key={"all"}
            onClick={() => setChnotKinds((_) => [])}
          >
            <Icon.Squirrel />
            <span>all</span>
          </DropdownMenuItem>
          {Object.values(ChnotKind).map((e) => (
            <DropdownMenuItem key={e}>
              <div className="flex items-center justify-between w-full">
                <div
                  className="flex items-center flex-1 gap-2 cursor-pointer"
                  onClick={() =>
                    setChnotKinds((kinds) => {
                      return [e, ...(kinds ?? [])];
                    })
                  }
                  tabIndex={0}
                  aria-label={`Select ${e}`}
                  onKeyDown={(event) =>
                    (event.key === "Enter" || event.key === " ") &&
                    setChnotKinds((kinds) => {
                      return [e, ...(kinds ?? [])];
                    })
                  }
                >
                  <ChnotKindIcon kind={e} />
                  <span className="truncate">{e}</span>
                </div>

                <Button
                  variant="ghost"
                  className="hover:bg-accent h-6"
                  onClick={(event) => {
                    event.stopPropagation();
                    setChnotKinds(() => [e]);
                  }}
                  aria-label={`Add ${e} to mkspace`}
                >
                  <Icon.Target />
                </Button>
              </div>
            </DropdownMenuItem>
          ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
