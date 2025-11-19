import { useEffect, useState } from 'react';

import { ChnotKind } from '../../po';
import { useChnotSingleStore } from '../../store';
import { ChnotSingleBodyMemo } from './body';
import ChnotSingleHeadbar from './header';

import { genTID, type TID } from '@/lib/id_util';
import { cn } from '@/lib/utils';

const ChnotSingleMain = ({ className }: { className?: string }) => {
  const { getMeta, setChangeCompCurOtid } = useChnotSingleStore((s) => {
    return {
      getMeta: s.getMeta,
      setChangeCompCurOtid: s.setChangeCompCurOtid,
    };
  });

  const [otid, setOtid] = useState<TID | undefined>(genTID());
  const [kind, setKind] = useState<ChnotKind | undefined>(undefined);

  useEffect(() => {
    setChangeCompCurOtid(setOtid);
  }, [setOtid]);

  useEffect(() => {
    if (otid) {
      setKind(getMeta(otid)?.meta.kind);
    } else {
      setKind(undefined);
    }
  }, [otid]);

  console.log('otid, kind', otid, kind);

  return (
    <main className={cn('relative flex flex-col overflow-y-auto', className)}>
      <ChnotSingleHeadbar
        className={'sticky top-0 left-0'}
        onNew={() => {
          setOtid(genTID());
        }}
        setKind={(kind: ChnotKind) => {
          setKind(kind);
        }}
        otid={otid}
      />
      {otid && <ChnotSingleBodyMemo key={otid} otid={otid} kind={kind ?? ChnotKind.MDWT} />}
    </main>
  );
};

export default ChnotSingleMain;
