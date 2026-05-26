declare module "@joplin/turndown" {
  interface TurndownOptions {
    headingStyle?: "setext" | "atx";
    hr?: string;
    bulletListMarker?: "-" | "+" | "*";
    codeBlockStyle?: "indented" | "fenced";
    fence?: "```" | "~~~";
    emDelimiter?: "*" | "_";
    strongDelimiter?: "**" | "__";
    linkStyle?: "inlined" | "referenced";
    linkReferenceStyle?: "full" | "collapsed" | "shortcut";
  }

  interface Rule {
    filter:
      | string
      | string[]
      | ((node: HTMLElement) => boolean);
    replacement?: (
      content: string,
      node: HTMLElement,
      options: TurndownOptions,
    ) => string;
  }

  class TurndownService {
    constructor(options?: TurndownOptions);
    use(plugin: Array<(service: TurndownService) => void>): this;
    addRule(name: string, rule: Rule): this;
    turndown(html: string | HTMLElement): string;
  }

  export default TurndownService;
}

declare module "@joplin/turndown-plugin-gfm" {
  import type TurndownService from "@joplin/turndown";

  export function tables(service: TurndownService): void;
  export function strikethrough(service: TurndownService): void;
  export function highlightedCodeBlock(service: TurndownService): void;
  export function taskListItems(service: TurndownService): void;
  export function gfm(service: TurndownService): void;
}
