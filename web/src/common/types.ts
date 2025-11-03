export enum SaveState {
  Initial,
  Dirty,
  Saved,
  Saving,
  Error,
}

export type PageRsp<T> = {
  data: T[];
  has_next: boolean;
  next_start: number;
};
