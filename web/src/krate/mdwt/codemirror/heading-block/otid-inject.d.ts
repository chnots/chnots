import { ViewPlugin, type ViewUpdate } from "@codemirror/view";
export declare function otidInjector(genTID: () => number): ViewPlugin<{
    update(update: ViewUpdate): void;
}, undefined>;
