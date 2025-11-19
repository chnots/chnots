import type { TID } from '@/lib/id_util';
import type { Varchar } from '@/lib/types';

export type KSpace = {
  name: Varchar<500>;
  color: Varchar<100>;
  managers: string[];
  tid: TID;
  public_access: boolean;
};
