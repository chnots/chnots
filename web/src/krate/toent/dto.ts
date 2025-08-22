type PossibleToent = object;
type PossibleScore = number;

export type ToentGuessReq = {
  input: string;
};

export type GuessElem<T> = {
  toent: T;
  score: PossibleScore;
};
export type ToentGuessRsp<T> = {
  toents: T[];
};
