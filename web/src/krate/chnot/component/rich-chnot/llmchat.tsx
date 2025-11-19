import { createRef, useEffect, useState } from 'react';

import Fullscreen from './fullscreen';

import type { RichPropProps } from './rich-chnot';
import { SaveState } from '@/common/types';
import SessionContainer, {
  type LLMChatContextProps,
  LLMChatEditorProvider,
} from '@/krate/llmchat/component/session';
import { llmchatSessionRecordFetch } from '@/krate/llmchat/service';
import { useLLMChatStore } from '@/krate/llmchat/store';
import type { TID } from '@/lib/id_util';

const LLMChatChnot = ({
  otid,
  fullscreen,
  readonly,
  onPostSave,
  onSetFullscreen,
}: RichPropProps) => {
  const persistedIds = createRef<Set<TID>>();

  const [props, setProps] = useState<LLMChatContextProps | undefined>(undefined);
  const { refreshTemplates } = useLLMChatStore();

  useEffect(() => {
    refreshTemplates();
  }, []);

  // Used to load from database.
  useEffect(() => {
    (async () => {
      persistedIds.current = new Set();

      if (otid) {
        const { session, records } = await llmchatSessionRecordFetch({
          session_otid: otid,
        });
        if (session) {
          setProps(() => {
            persistedIds.current!.add(session.otid);
            records.forEach((r) => persistedIds.current!.add(r.otid));
            return {
              sessionOtid: otid,
              records: records,
              session: session,
              persistedIds: persistedIds,
            };
          });
        } else {
          setProps({
            sessionOtid: otid,
            persistedIds: persistedIds,
          });
        }
      }
    })();
  }, [otid]);

  console.log('render llmchat', props, readonly, fullscreen);
  return (
    props && (
      <LLMChatEditorProvider props={props}>
        {fullscreen ? (
          <Fullscreen onSetFullscreen={onSetFullscreen}>
            <SessionContainer
              onPostSave={(s) => {
                onPostSave({
                  saveState: SaveState.Saved,
                  title: s.title,
                });
              }}
              readonly={false}
            />
          </Fullscreen>
        ) : (
          <div className="w-full h-full">
            {readonly ? (
              <SessionContainer readonly={true} />
            ) : (
              <SessionContainer
                readonly={false}
                onPostSave={(s) => {
                  onPostSave({
                    saveState: SaveState.Saved,
                    title: s.title,
                  });
                }}
              />
            )}
          </div>
        )}
      </LLMChatEditorProvider>
    )
  );
};

export default LLMChatChnot;
