import type { ContentBlock } from "./po";
import type { LLMChatRecordVO } from "./vo";

export function recordToMarkdown(record: LLMChatRecordVO): string {
  return contentBlocksToMarkdown(record.content);
}

export function contentBlocksToMarkdown(blocks: ContentBlock[]): string {
  const parts: string[] = [];

  for (const block of blocks) {
    switch (block.type) {
      case "thinking":
        parts.push(
          `<details>\n<summary>Thinking</summary>\n\n${block.data}\n\n</details>`,
        );
        break;
      case "content":
        parts.push(block.data);
        break;
      case "error":
        parts.push(`> \u274C ${block.data}`);
        break;
      case "image":
        parts.push(`![${block.filename ?? "image"}](${block.data})`);
        break;
      case "file":
        parts.push(`[${block.filename ?? "file"}](${block.data})`);
        break;
    }
  }

  return parts.join("\n\n");
}
