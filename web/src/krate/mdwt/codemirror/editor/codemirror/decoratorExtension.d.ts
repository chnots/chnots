import { ViewPlugin, DecorationSet, ViewUpdate } from '@codemirror/view';
declare const decoratorExtension: ViewPlugin<{
    decorations: DecorationSet;
    update(viewUpdate: ViewUpdate): void;
}, undefined>;
export default decoratorExtension;
