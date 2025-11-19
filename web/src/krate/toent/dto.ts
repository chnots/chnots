import type { MdwtToent } from './po';
import type { TID } from '@/lib/id_util';

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

export type Toents = {
  toent_inst_map: Record<TID, MdwtToent[]>;
};
