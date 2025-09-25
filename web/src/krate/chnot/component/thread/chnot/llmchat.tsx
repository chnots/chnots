import { SaveState } from "@/common/types";
import { ChnotKind } from "@/krate/chnot/po";
import { ChnotChromeProps } from "./chrome";
import LLMChatTemplateList from "@/krate/llmchat/component/template-list";
import { LLMChatTemplate } from "@/krate/llmchat/po";
import { createRef, useEffect, useState } from "react";
import SessionContainer, {
  LLMChatEditorProvider,
  LLMChatContextProps,
} from "@/krate/llmchat/component/session";
import { genTID, TID } from "@/lib/id_util";
import { useLLMChatStore } from "@/krate/llmchat/store";
import { Button } from "@/common/component/ui/button";
import { llmchatSessionRecordFetch } from "@/krate/llmchat/service";

const LLMChatChnot = ({ chnotOtid, kindId, onPostSave }: ChnotChromeProps) => {
  const [props, setProps] = useState<LLMChatContextProps | undefined>(
    undefined,
  );
  const [sessionOtid] = useState(kindId ? parseInt(kindId) : genTID());
  const { refreshTemplates } = useLLMChatStore();
  const [fullscreen, setFullscreen] = useState(false);

  console.log("fullscreen", fullscreen);

  useEffect(() => {
    refreshTemplates();
  }, []);

  // Used to load from database.
  useEffect(() => {
    (async () => {
      if (sessionOtid) {
        const rsp = await llmchatSessionRecordFetch(sessionOtid);
        if (rsp.session) {
          setProps(() => {
            const pids = new Set<TID>();

            if (rsp.session) {
              rsp.records.forEach((r) => {
                pids.add(r.otid);
              });
              pids.add(rsp.session.otid);
            }
            const pidRef = createRef<Set<TID>>();
            pidRef.current = pids;
            return {
              sessionOtid: sessionOtid,
              records: rsp.records,
              session: rsp.session,
              persistedIds: pidRef,
            };
          });
        }
      }
    })();
  }, [sessionOtid]);
  console.log("props", props);

  return props ? (
    <LLMChatEditorProvider props={props}>
      {fullscreen ? (
        <div className="w-screen h-screen z-50 flex flex-col fixed bottom-0 left-0 m-0 p-0 bg-background items-center">
          <Button onClick={() => setFullscreen(false)}>Close</Button>
          <SessionContainer
            onPostSave={(session) => {
              console.log("session post save");
              onPostSave({
                saveState: SaveState.Saved,
                data: {
                  chnotOtid,
                  kind: ChnotKind.LLMChat,
                  kindId: session.otid.toString(),
                },
              });
            }}
            viewMode={false}
          />
        </div>
      ) : (
        <div className="m-0 p-0">
          <Button
            onClick={() => {
              setFullscreen(true);
            }}
          >
            Fullscreen
          </Button>
          <SessionContainer viewMode={true} />
        </div>
      )}
    </LLMChatEditorProvider>
  ) : (
    <div>
      <LLMChatTemplateList
        onClickTemplate={(template: LLMChatTemplate) => {
          setFullscreen(true);

          setProps((props) => {
            if (props) {
              return {
                ...props,
                template: template,
              };
            } else {
              const ref = createRef<Set<TID> | null>();
              ref.current = new Set();
              const props: LLMChatContextProps = {
                sessionOtid,
                persistedIds: ref,
                template: template,
              };
              return props;
            }
          });
        }}
      />
    </div>
  );
};

export default LLMChatChnot;
