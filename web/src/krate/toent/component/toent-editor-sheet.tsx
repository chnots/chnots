import { Badge } from "@/common/component/ui/badge";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "@/common/component/ui/sheet";
import { SaveState } from "@/common/types";
import MdwtChnot from "@/krate/chnot/component/rich-chnot/mdwt";
import { ChnotKind } from "@/krate/chnot/po";
import { chnotMetaCommit } from "@/krate/chnot/service";
import type { TID } from "@/lib/id_util";
import type { ToentEditorTarget } from "./toent-page-shared";

export function ToentEditorSheet({
  editorTarget,
  currentKSpace,
  ensuredMetaOtids,
  onClose,
}: {
  editorTarget?: ToentEditorTarget;
  currentKSpace: string;
  ensuredMetaOtids: Set<TID>;
  onClose: () => void;
}) {
  return (
    <Sheet
      open={editorTarget !== undefined}
      onOpenChange={(open) => {
        if (!open) {
          onClose();
        }
      }}
    >
      <SheetContent side="right" className="w-full p-0 sm:max-w-3xl">
        {editorTarget ? (
          <div className="flex h-full flex-col">
            <SheetHeader className="border-b pb-3">
              <SheetTitle>{editorTarget.title}</SheetTitle>
              <SheetDescription>{editorTarget.dateText}</SheetDescription>
              <div className="flex items-center gap-2 text-xs">
                <Badge variant="outline">State: {editorTarget.state}</Badge>
                {editorTarget.priority ? (
                  <Badge variant="secondary">
                    Priority: P{editorTarget.priority}
                  </Badge>
                ) : null}
              </div>
            </SheetHeader>
            <div className="min-h-0 flex-1 overflow-auto p-2">
              <div className="h-full min-h-[55vh]">
                <MdwtChnot
                  otid={editorTarget.chnotOtid}
                  readonly={false}
                  fullscreen={false}
                  onPostSave={async (arg) => {
                    if (arg.saveState !== SaveState.Saved) {
                      return;
                    }

                    if (ensuredMetaOtids.has(arg.otid)) {
                      return;
                    }

                    await chnotMetaCommit({
                      metas: [
                        {
                          otid: arg.otid,
                          kind: ChnotKind.MDWT,
                          kspace: currentKSpace,
                        },
                      ],
                    });
                    ensuredMetaOtids.add(arg.otid);
                  }}
                />
              </div>
            </div>
          </div>
        ) : null}
      </SheetContent>
    </Sheet>
  );
}
