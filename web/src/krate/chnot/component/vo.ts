import type { ChnotKind } from '../po';
import type { TID } from '@/lib/id_util';

export type ChnotMetaKind = {
  chnotOtid: TID;
  kind: ChnotKind;
};
