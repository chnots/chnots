import Icon from "@/common/component/icon";
import { Button } from "@/common/component/ui/button";
import { SidebarTrigger } from "@/common/component/ui/sidebar";
import type { TID } from "@/lib/id_util";
import { cn } from "@/lib/utils";
import { ChnotKind } from "../../po";
import { useChnotSingleStore } from "../../store";
import { ChnotKindIcon } from "../kind-icon";
import { useRef } from "react";
import { mdwtCommit } from "@/krate/mdwt/service";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";
import { MdwtEditorMemo } from "@/krate/mdwt/component/mdwt-editor";

const ChnotSingleHeadbar = ({
  otid,
  onNew,
  setKind,
  className,
}: {
  otid?: TID;
  onNew: () => void;
  setKind: (kind: ChnotKind) => void;
  className?: string;
}) => {
  const { getMeta, overwritePart } = useChnotSingleStore((s) => {
    return {
      getMeta: s.getMeta,
      overwritePart: s.overwritePart,
    };
  });

  const meta = otid ? getMeta(otid) : undefined;
  const titleRef = useRef<string>(meta?.title);

  return (
    <div
      className={cn(
        "w-full flex align-center items-center p-1 space-x-1",
        className,
      )}
    >
      <SidebarTrigger />
      <div className="m-1">
        <Button onClick={onNew}>
          <Icon.BadgePlusIcon />
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
              <Icon.Captions />
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
    </div>
  );
};

export default ChnotSingleHeadbar;
