import type { SaveState } from "@/common/types";
import type { TID } from "@/lib/id_util";
import type { ChnotKind } from "../../po";

export type PostSaveArg = {
  otid: TID;
  saveState: SaveState;
  kind: ChnotKind;
  title?: string;
};

export type RichPropProps = {
  otid: TID;
  readonly?: boolean;
  fullscreen: boolean;
  onSetFullscreen?: (flag: boolean) => void;
  onPostSave: (arg: PostSaveArg) => Promise<void>;
  disableHeaderActions?: boolean;
};
