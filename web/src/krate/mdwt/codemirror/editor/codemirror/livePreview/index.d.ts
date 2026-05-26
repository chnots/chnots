import { StateField } from '@codemirror/state';
import { type DecorationSet } from '@codemirror/view';
declare const livePreview: () => (import("@codemirror/state").Extension | StateField<DecorationSet>)[];
export default livePreview;
