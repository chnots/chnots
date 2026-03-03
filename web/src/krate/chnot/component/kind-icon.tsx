import type { LucideProps } from "lucide-react";
import {
  Bot,
  Brain,
  File,
  Flower,
  LineSquiggle,
  Table,
  Text,
  TextCursor,
} from "lucide-react";
import { ChnotKind } from "../po";

export const ChnotKindIcon = ({
  kind,
  ...rest
}: { kind: ChnotKind } & LucideProps) => {
  return kind === ChnotKind.MDWT ? (
    <Text {...rest} />
  ) : kind === ChnotKind.ExcalidrawV1 ? (
    <Flower {...rest} />
  ) : kind === ChnotKind.KFileV1 ? (
    <File {...rest} />
  ) : kind === ChnotKind.KTab ? (
    <Table {...rest} />
  ) : kind === ChnotKind.LLMChat ? (
    <Bot {...rest} />
  ) : kind === ChnotKind.MindMapV1 ? (
    <Brain {...rest} />
  ) : kind === ChnotKind.ThreadV1 ? (
    <LineSquiggle {...rest} />
  ) : (
    <TextCursor {...rest} />
  );
};
