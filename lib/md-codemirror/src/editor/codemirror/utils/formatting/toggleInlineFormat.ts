import { syntaxTree } from '@codemirror/language';
import {
  EditorSelection,
  EditorState,
  SelectionRange,
  TransactionSpec,
} from '@codemirror/state';

type InlineFormatConfig = {
  marker: string;
  nodeNames: string[];
  markerNodeName: string;
};

const FORMAT_CONFIGS: Record<string, InlineFormatConfig> = {
  bold: {
    marker: '**',
    nodeNames: ['StrongEmphasis'],
    markerNodeName: 'EmphasisMark',
  },
  italic: {
    marker: '*',
    nodeNames: ['Emphasis'],
    markerNodeName: 'EmphasisMark',
  },
  strikethrough: {
    marker: '~~',
    nodeNames: ['Strikethrough'],
    markerNodeName: 'StrikethroughMark',
  },
  inlineCode: {
    marker: '`',
    nodeNames: ['InlineCode'],
    markerNodeName: 'CodeMark',
  },
  highlight: {
    marker: '==',
    nodeNames: ['Highlight'],
    markerNodeName: 'HighlightMark',
  },
};

const findEnclosingNode = (
  state: EditorState,
  pos: number,
  nodeNames: string[],
): { from: number; to: number } | null => {
  let result: { from: number; to: number } | null = null;

  syntaxTree(state).iterate({
    enter: (node) => {
      for (const name of nodeNames) {
        if (node.name === name && node.from <= pos && pos <= node.to) {
          if (!result || node.to - node.from < result.to - result.from) {
            result = { from: node.from, to: node.to };
          }
        }
      }
    },
  });

  return result;
};

const findMarkerPositions = (
  state: EditorState,
  from: number,
  to: number,
  markerNodeName: string,
): {
  openFrom: number;
  openTo: number;
  closeFrom: number;
  closeTo: number;
} | null => {
  const markers: { from: number; to: number }[] = [];

  syntaxTree(state).iterate({
    from,
    to,
    enter: (node) => {
      if (node.name === markerNodeName) {
        markers.push({ from: node.from, to: node.to });
      }
    },
  });

  if (markers.length >= 2) {
    return {
      openFrom: markers[0].from,
      openTo: markers[0].to,
      closeFrom: markers[markers.length - 1].from,
      closeTo: markers[markers.length - 1].to,
    };
  }

  return null;
};

const isFormatApplied = (
  state: EditorState,
  from: number,
  to: number,
  config: InlineFormatConfig,
): boolean => {
  let found = false;
  syntaxTree(state).iterate({
    from,
    to,
    enter: (node) => {
      for (const name of config.nodeNames) {
        if (node.name === name && node.from <= from && node.to >= to) {
          found = true;
          return false;
        }
      }
      return true;
    },
  });
  return found;
};

export const toggleInlineFormat = (
  state: EditorState,
  formatName: string,
): TransactionSpec => {
  const config = FORMAT_CONFIGS[formatName];
  if (!config) return { changes: [] };

  const marker = config.marker;
  const markerLen = marker.length;

  return state.changeByRange((sel: SelectionRange) => {
    const { from, to, empty } = sel;

    if (empty) {
      const node = findEnclosingNode(state, from, config.nodeNames);

      if (node) {
        const markerPositions = findMarkerPositions(
          state,
          node.from,
          node.to,
          config.markerNodeName,
        );
        if (markerPositions) {
          return {
            changes: [
              {
                from: markerPositions.openFrom,
                to: markerPositions.openTo,
                insert: '',
              },
              {
                from: markerPositions.closeFrom,
                to: markerPositions.closeTo,
                insert: '',
              },
            ],
            range: EditorSelection.cursor(
              from - (from > markerPositions.openTo ? markerLen : 0),
            ),
          };
        }
      }

      return {
        changes: [{ from, insert: marker + marker }],
        range: EditorSelection.cursor(from + markerLen),
      };
    }

    if (isFormatApplied(state, from, to, config)) {
      const node = findEnclosingNode(state, from, config.nodeNames);
      if (node) {
        const markerPositions = findMarkerPositions(
          state,
          node.from,
          node.to,
          config.markerNodeName,
        );
        if (markerPositions) {
          let newFrom = from - markerLen;
          let newTo = to - markerLen * 2;
          if (from <= markerPositions.openTo)
            newFrom = markerPositions.openFrom;
          if (to >= markerPositions.closeFrom)
            newTo = markerPositions.closeTo - markerLen * 2;

          return {
            changes: [
              {
                from: markerPositions.openFrom,
                to: markerPositions.openTo,
                insert: '',
              },
              {
                from: markerPositions.closeFrom,
                to: markerPositions.closeTo,
                insert: '',
              },
            ],
            range: EditorSelection.range(newFrom, newTo),
          };
        }
      }
    }

    const text = state.sliceDoc(from, to);
    return {
      changes: [
        { from, insert: marker },
        { from: to, insert: marker },
      ],
      range: EditorSelection.range(from + markerLen, to + markerLen),
    };
  });
};

export const toggleBold = (state: EditorState): TransactionSpec =>
  toggleInlineFormat(state, 'bold');

export const toggleItalic = (state: EditorState): TransactionSpec =>
  toggleInlineFormat(state, 'italic');

export const toggleStrikethrough = (state: EditorState): TransactionSpec =>
  toggleInlineFormat(state, 'strikethrough');

export const toggleInlineCode = (state: EditorState): TransactionSpec =>
  toggleInlineFormat(state, 'inlineCode');

export const toggleHighlight = (state: EditorState): TransactionSpec =>
  toggleInlineFormat(state, 'highlight');

export default toggleInlineFormat;
