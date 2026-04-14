import { StateField, type Transaction } from '@codemirror/state';
import { Decoration, type DecorationSet, EditorView } from '@codemirror/view';
import { buildDecorations } from './decoration-builder';
import { tableNavigation } from './table-navigation';

const livePreviewField = StateField.define<DecorationSet>({
  create(state) {
    return buildDecorations(state);
  },
  update(_decos: DecorationSet, tr: Transaction): DecorationSet {
    if (tr.docChanged || tr.selection) {
      return buildDecorations(tr.state);
    }
    return _decos.map(tr.changes);
  },
  provide: (f) => EditorView.decorations.from(f),
});

const livePreviewTheme = EditorView.baseTheme({
  '&light .cm-livePreview-heading': {
    fontWeight: '600',
    fontFamily:
      'ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    letterSpacing: '-0.025em',
  },
  '&dark .cm-livePreview-heading': {
    fontWeight: '600',
    fontFamily:
      'ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    letterSpacing: '-0.025em',
  },
  '.cm-livePreview-h1': { fontSize: '1.5em' },
  '.cm-livePreview-h2': { fontSize: '1.25em' },
  '.cm-livePreview-h3': { fontSize: '1.125em' },
  '.cm-livePreview-h4': { fontSize: '1.05em' },
  '.cm-livePreview-h5': { fontSize: '1em' },
  '.cm-livePreview-h6': { fontSize: '0.95em' },

  '&light .cm-livePreview-math': { color: '#0369a1' },
  '&dark .cm-livePreview-math': { color: '#7dd3fc' },
  '.cm-livePreview-math-block': {
    textAlign: 'center',
    padding: '0.5em 0',
    overflowX: 'auto',
  },
  '.cm-livePreview-math-inline': {
    display: 'inline',
  },
  '.cm-livePreview-math-raw': {
    fontFamily: 'monospace',
    fontSize: '0.9em',
  },

  '.cm-livePreview-image': {
    margin: '0.25em 0',
  },
  '&light .cm-livePreview-image-error': {
    color: '#737373',
    fontStyle: 'italic',
    fontSize: '0.9em',
  },
  '&dark .cm-livePreview-image-error': {
    color: '#a1a1a1',
    fontStyle: 'italic',
    fontSize: '0.9em',
  },

  '.cm-livePreview-hr': {
    border: 'none',
    borderTop: '1px solid #e5e5e5',
    margin: '0.75em 0',
  },

  '.cm-livePreview-table-wrapper': {
    overflowX: 'auto',
    margin: '0.5em 0',
  },
  '.cm-livePreview-table': {
    borderCollapse: 'collapse',
    width: '100%',
    fontSize: '0.875em',
    fontFamily:
      'ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
  },
  '&light .cm-livePreview-table th, &light .cm-livePreview-table td': {
    border: '1px solid #e5e5e5',
    padding: '0.375rem 0.75rem',
  },
  '&dark .cm-livePreview-table th, &dark .cm-livePreview-table td': {
    border: '1px solid #363636',
    padding: '0.375rem 0.75rem',
  },
  '&light .cm-livePreview-table th': {
    backgroundColor: '#f5f5f5',
    fontWeight: '600',
  },
  '&dark .cm-livePreview-table th': {
    backgroundColor: '#2a2a2a',
    fontWeight: '600',
  },

  '.cm-livePreview-checkbox': {
    marginRight: '0.25em',
    verticalAlign: 'middle',
    pointerEvents: 'auto',
    cursor: 'default',
  },

  '.cm-hiddenMark': {
    fontSize: '0',
    lineHeight: '0',
    display: 'inline',
    opacity: '0',
    width: '0',
    height: '0',
    overflow: 'hidden',
  },
});

const livePreview = () => [livePreviewField, livePreviewTheme, tableNavigation()];

export default livePreview;
