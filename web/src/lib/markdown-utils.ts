// adopted from https://github.com/docmost/docmost/blob/main/apps/server/src/integrations/export/turndown-utils.ts

import TurndownService from "@joplin/turndown";
import * as TurndownPluginGfm from "@joplin/turndown-plugin-gfm";

export function html2mdAsync(html: string): Promise<string> {
  return Promise.resolve(html2md(html));
}

export function html2md(html: string): string {
  const turndownService = new TurndownService({
    headingStyle: "atx",
    codeBlockStyle: "fenced",
    hr: "---",
    bulletListMarker: "-",
  });
  const tables = TurndownPluginGfm.tables;
  const strikethrough = TurndownPluginGfm.strikethrough;
  const highlightedCodeBlock = TurndownPluginGfm.highlightedCodeBlock;

  turndownService.use([
    tables,
    strikethrough,
    highlightedCodeBlock,
    taskList,
    callout,
    preserveDetail,
    listParagraph,
    mathInline,
    mathBlock,
  ]);

  return turndownService.turndown(html).replaceAll("<br>", " ");
}

function listParagraph(turndownService: TurndownService) {
  turndownService.addRule("paragraph", {
    filter: ["p"],
    replacement: (content, node) => {
      if (node.parentElement?.nodeName === "LI") {
        return content;
      }

      return `\n\n${content}\n\n`;
    },
  });
}

function callout(turndownService: TurndownService) {
  turndownService.addRule("callout", {
    filter: (node) =>
      node.nodeName === "DIV" && node.getAttribute("data-type") === "callout",
    replacement: (content, node) => {
      const calloutType = node.getAttribute("data-callout-type");
      return `\n\n:::${calloutType}\n${content.trim()}\n:::\n\n`;
    },
  });
}

function taskList(turndownService: TurndownService) {
  turndownService.addRule("taskListItem", {
    filter: (node) =>
      node.getAttribute("data-type") === "taskItem" &&
      node.parentNode?.nodeName === "UL",
    replacement: (content, node) => {
      const checkbox = node.querySelector(
        'input[type="checkbox"]',
      ) as HTMLInputElement;
      const isChecked = checkbox.checked;

      return `- ${isChecked ? "[x]" : "[ ]"}  ${content.trim()} \n`;
    },
  });
}

function preserveDetail(turndownService: TurndownService) {
  turndownService.addRule("preserveDetail", {
    filter: (node) => node.nodeName === "DETAILS",
    replacement: (_content, node) => {
      const summary = node.querySelector(":scope > summary");
      let detailSummary = "";

      if (summary) {
        detailSummary = `<summary>${turndownService.turndown(summary.innerHTML)}</summary>`;
        summary.remove();
      }

      const detailsContent = turndownService.turndown(node.innerHTML);
      return `\n<details>\n${detailSummary}\n\n${detailsContent}\n\n</details>\n`;
    },
  });
}

function mathInline(turndownService: TurndownService) {
  turndownService.addRule("mathInline", {
    filter: (node) =>
      node.nodeName === "SPAN" &&
      node.getAttribute("data-type") === "mathInline",
    replacement: (content) => `$${content}$`,
  });
}

function mathBlock(turndownService: TurndownService) {
  turndownService.addRule("mathBlock", {
    filter: (node) =>
      node.nodeName === "DIV" && node.getAttribute("data-type") === "mathBlock",
    replacement: (content) => `\n$$${content}$$\n`,
  });
}
