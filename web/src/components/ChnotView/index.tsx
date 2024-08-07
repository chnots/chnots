export interface ChnotViewState {
  isUploadingResource: boolean;
  isRequesting: boolean;
  isComposing: boolean;
}

export enum ChnotViewMode {
  Editor = "editor",
  Preview = "preview",
  Both = "both",
}
