import { BadgePlus, Captions, List } from "lucide-react";
import { useRef } from "react";
import { Link, useLocation } from "react-router-dom";
import { Button } from "@/common/component/ui/button";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";
import { SidebarTrigger } from "@/common/component/ui/sidebar";
import { MdwtEditorMemo } from "@/krate/mdwt/component/mdwt-editor";
import { mdwtCommit } from "@/krate/mdwt/service";
import type { TID } from "@/lib/id_util";
import { cn } from "@/lib/utils";
import { RoutePaths } from "@/router";
import { ChnotKind } from "../../po";
import { useChnotStore } from "../../store";
import { ChnotKindIcon } from "../kind-icon";

const ChnotHeadbar = ({
  otid,
  onNew,
  setKind,
  className,
  hideSidebar = false,
}: {
  otid?: TID;
  onNew: () => void;
  setKind: (kind: ChnotKind) => void;
  className?: string;
  hideSidebar?: boolean;
}) => {
  const location = useLocation();
  const allChnotsSearch = (() => {
    const params = new URLSearchParams(location.search);
    params.delete("otid");
    const search = params.toString();
    return search.length > 0 ? `?${search}` : "";
  })();
  const { overwritePart, mapByOtid, headerActions } = useChnotStore((s) => {
    return {
      overwritePart: s.overwritePart,
      mapByOtid: s.mapByOtid,
      headerActions: s.headerActions,
    };
  });

  const meta = otid ? mapByOtid.cache.get(otid) : undefined;
  const titleRef = useRef<string>(meta?.title);

  return (
    <div
      className={cn(
        "flex h-14 w-full items-center gap-1 border-b px-4",
        className,
      )}
    >
      {hideSidebar ? (
        <Button asChild variant="ghost" title="Show all chnots">
          <Link to={{ pathname: RoutePaths.Chnots, search: allChnotsSearch }}>
            <List />
          </Link>
        </Button>
      ) : (
        <SidebarTrigger />
      )}
      <div className="m-1">
        <Button onClick={onNew}>
          <BadgePlus />
        </Button>
      </div>
      {!meta && (
        <div className="rounded-md border p-0 m-0">
          {Object.values(ChnotKind).map((kind) => {
            return (
              <Button
                key={kind}
                onClick={() => {
                  setKind(kind);
                }}
                variant={"ghost"}
              >
                <ChnotKindIcon kind={kind} />
              </Button>
            );
          })}
        </div>
      )}
      {meta && meta.meta.kind !== ChnotKind.MDWT && (
        <Popover
          onOpenChange={async (open) => {
            if (!open && titleRef.current && titleRef.current !== meta.title) {
              await mdwtCommit({
                mdwt: {
                  otid: meta.meta.otid,
                  content: titleRef.current,
                },
              });
              overwritePart(meta.meta.otid, { title: titleRef.current });
            }
          }}
        >
          <PopoverTrigger asChild>
            <Button>
              <Captions />
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-auto p-2" align="start">
            <MdwtEditorMemo
              content={meta.title?.trim()}
              foldGutter={false}
              onContentChange={(content: string): void => {
                titleRef.current = content;
              }}
            />
          </PopoverContent>
        </Popover>
      )}
      {headerActions.length > 0 && (
        <div className="ml-auto flex items-center gap-1">
          {headerActions.map((ha) => (
            <div key={ha.key}>{ha.actions}</div>
          ))}
        </div>
      )}
    </div>
  );
};

export default ChnotHeadbar;
