import { StateField } from "@codemirror/state";
import { parseHeadings } from "./block-parser";
import type { HeadingBlock } from "./block-model";

export const headingBlockField = StateField.define<HeadingBlock[]>({
  create(state) {
    return parseHeadings(state);
  },
  update(_blocks, tr) {
    if (tr.docChanged) {
      return parseHeadings(tr.state);
    }
    return _blocks;
  },
});
