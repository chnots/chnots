import { syntaxTree } from '@codemirror/language';
import { type EditorState, RangeSetBuilder } from '@codemirror/state';
import { Decoration, type DecorationSet } from '@codemirror/view';
import { ImageWidget, parseImage } from './widgets/image-widget';
import { BlockMathWidget, InlineMathWidget } from './widgets/math-widget';
import { CheckboxWidget, HorizontalRuleWidget } from './widgets/misc-widgets';
import { TableWidget } from './widgets/table-widget';

type DecorationEntry = {
  from: number;
  to: number;
  decoration: Decoration;
};

const noSpellCheck = { spellcheck: 'false', autocorrect: 'false' };

const hideMarkDecoration = Decoration.mark({ class: 'cm-hiddenMark' });

const lineDecorations: Record<string, Decoration> = {
  FencedCode: Decoration.line({
    attributes: { class: 'cm-codeBlock cm-regionFirstLine cm-regionLastLine', ...noSpellCheck },
  }),
  CodeBlock: Decoration.line({
    attributes: { class: 'cm-codeBlock cm-regionFirstLine cm-regionLastLine', ...noSpellCheck },
  }),
  Blockquote: Decoration.line({
    attributes: { class: 'cm-blockQuote' },
  }),
  OrderedList: Decoration.line({
    attributes: { class: 'cm-orderedList' },
  }),
  BulletList: Decoration.line({
    attributes: { class: 'cm-unorderedList' },
  }),
  ListItem: Decoration.line({
    attributes: { class: 'cm-listItem' },
  }),
  TableHeader: Decoration.line({
    attributes: { class: 'cm-tableHeader' },
  }),
  TableDelimiter: Decoration.line({
    attributes: { class: 'cm-tableDelimiter' },
  }),
  TableRow: Decoration.line({
    attributes: { class: 'cm-tableRow' },
  }),
};

const markDecorations: Record<string, Decoration> = {
  InlineCode: Decoration.mark({
    attributes: { class: 'cm-inlineCode', ...noSpellCheck },
  }),
  URL: Decoration.mark({
    attributes: { class: 'cm-url', ...noSpellCheck },
  }),
  HTMLTag: Decoration.mark({
    attributes: { class: 'cm-htmlTag', ...noSpellCheck },
  }),
  TagName: Decoration.mark({
    attributes: { class: 'cm-htmlTag', ...noSpellCheck },
  }),
  TaskMarker: Decoration.mark({
    attributes: { class: 'cm-taskMarker' },
  }),
  Strikethrough: Decoration.mark({
    attributes: { class: 'cm-strike' },
  }),
  Highlight: Decoration.mark({
    attributes: { class: 'cm-highlighted' },
  }),
  StrongEmphasis: Decoration.mark({
    attributes: { class: 'cm-strong' },
  }),
  Emphasis: Decoration.mark({
    attributes: { class: 'cm-emphasis' },
  }),
};

const headingNodeNames = [
  'ATXHeading1', 'ATXHeading2', 'ATXHeading3',
  'ATXHeading4', 'ATXHeading5', 'ATXHeading6',
];

function getHeadingLevel(name: string): number {
  const idx = headingNodeNames.indexOf(name);
  return idx >= 0 ? idx + 1 : 0;
}

function isOnActiveLine(state: EditorState, from: number, to: number, activeLine: number): boolean {
  const startLine = state.doc.lineAt(from).number;
  const endLine = state.doc.lineAt(to).number;
  return startLine <= activeLine && activeLine <= endLine;
}

function hideHeadingPrefix(state: EditorState, nodeFrom: number, nodeTo: number, level: number, entries: DecorationEntry[]) {
  const text = state.sliceDoc(nodeFrom, nodeTo);
  const match = text.match(/^(#{1,6}\s+)/);
  if (match) {
    entries.push({
      from: nodeFrom,
      to: nodeFrom + match[1].length,
      decoration: hideMarkDecoration,
    });
  }
  entries.push({
    from: state.doc.lineAt(nodeFrom).from,
    to: state.doc.lineAt(nodeFrom).from,
    decoration: Decoration.line({
      attributes: { class: `cm-h${level} cm-headerLine cm-header` },
    }),
  });
}

export function buildDecorations(state: EditorState): DecorationSet {
  const activePos = state.selection.main.head;
  const activeLine = state.doc.lineAt(activePos).number;

  const entries: DecorationEntry[] = [];

  syntaxTree(state).iterate({
    enter: (node) => {
      const nodeFrom = node.from;
      const nodeTo = node.to;
      const onActive = isOnActiveLine(state, nodeFrom, nodeTo, activeLine);

      const headingLevel = getHeadingLevel(node.name);
      if (headingLevel > 0) {
        if (!onActive) {
          hideHeadingPrefix(state, nodeFrom, nodeTo, headingLevel, entries);
        } else {
          const line = state.doc.lineAt(nodeFrom);
          entries.push({
            from: line.from,
            to: line.from,
            decoration: Decoration.line({
              attributes: { class: `cm-h${headingLevel} cm-headerLine cm-header` },
            }),
          });
        }
        return;
      }

      if (node.name === 'HorizontalRule' && !onActive) {
        entries.push({
          from: nodeFrom,
          to: nodeTo,
          decoration: Decoration.replace({
            widget: new HorizontalRuleWidget(),
            block: true,
          }),
        });
        return;
      }

      if (node.name === 'Table' && !onActive) {
        const text = state.sliceDoc(nodeFrom, nodeTo);
        entries.push({
          from: nodeFrom,
          to: nodeTo,
          decoration: Decoration.replace({
            widget: new TableWidget(text, nodeFrom, nodeTo),
            block: true,
          }),
        });
        return false;
      }

      if (node.name === 'Image' && !onActive) {
        const text = state.sliceDoc(nodeFrom, nodeTo);
        const parsed = parseImage(text);
        if (parsed) {
          entries.push({
            from: nodeFrom,
            to: nodeTo,
            decoration: Decoration.replace({
              widget: new ImageWidget(parsed.url, parsed.alt),
            }),
          });
        }
        return;
      }

      if (node.name === 'InlineMath' && !onActive) {
        const text = state.sliceDoc(nodeFrom, nodeTo);
        const isDisplay = text.startsWith('$$');
        const latex = text.replace(/^\$+/, '').replace(/\$+$/, '');
        entries.push({
          from: nodeFrom,
          to: nodeTo,
          decoration: Decoration.replace({
            widget: new InlineMathWidget(latex, isDisplay),
          }),
        });
        return;
      }

      if (node.name === 'BlockMath' && !onActive) {
        const text = state.sliceDoc(nodeFrom, nodeTo);
        const latex = text.replace(/^\$\$/, '').replace(/\$\$$/, '').trim();
        entries.push({
          from: nodeFrom,
          to: nodeTo,
          decoration: Decoration.replace({
            widget: new BlockMathWidget(latex),
            block: true,
          }),
        });
        return;
      }

      if (node.name === 'TaskMarker' && !onActive) {
        const text = state.sliceDoc(nodeFrom, nodeTo);
        const checked = /\[x\]/i.test(text);
        entries.push({
          from: nodeFrom,
          to: nodeTo,
          decoration: Decoration.replace({
            widget: new CheckboxWidget(checked),
          }),
        });
        return;
      }

      if (node.name === 'EmphasisMark' && !onActive) {
        entries.push({
          from: nodeFrom,
          to: nodeTo,
          decoration: hideMarkDecoration,
        });
        return;
      }

      if (lineDecorations[node.name]) {
        let pos = nodeFrom;
        while (pos <= nodeTo) {
          const line = state.doc.lineAt(pos);
          entries.push({
            from: line.from,
            to: line.from,
            decoration: lineDecorations[node.name],
          });
          pos = line.to + 1;
        }
      }

      if (markDecorations[node.name]) {
        entries.push({
          from: nodeFrom,
          to: nodeTo,
          decoration: markDecorations[node.name],
        });
      }
    },
  });

  entries.sort((a, b) => {
    const posCmp = a.from - b.from;
    if (posCmp !== 0) return posCmp;
    return (a.to - a.from) - (b.to - b.from);
  });

  const builder = new RangeSetBuilder<Decoration>();
  for (const entry of entries) {
    builder.add(entry.from, entry.to, entry.decoration);
  }
  return builder.finish();
}
