import type { LucideProps } from "lucide-react";
import Icon from "@/common/component/icon";
import { ChnotKind } from "../po";

export const ChnotKindIcon = ({
  kind,
  ...rest
}: { kind: ChnotKind } & LucideProps) => {
  return kind === ChnotKind.MDWT ? (
    <Icon.Text {...rest} />
  ) : kind === ChnotKind.ExcalidrawV1 ? (
    <Icon.Flower {...rest} />
  ) : kind === ChnotKind.KFileV1 ? (
    <Icon.File {...rest} />
  ) : kind === ChnotKind.KTab ? (
    <Icon.Table {...rest} />
  ) : kind === ChnotKind.LLMChat ? (
    <Icon.Bot {...rest} />
  ) : (
    <Icon.TextCursor {...rest} />
  );
};
