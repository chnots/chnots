import { SaveState } from "@/common/types";
import { RichPropProps } from "./rich-chnot";
import LLMChatTemplateList from "@/krate/llmchat/component/template-list";
import { LLMChatTemplate } from "@/krate/llmchat/po";
import { createRef, useEffect, useState } from "react";
import SessionContainer, {
  LLMChatEditorProvider,
  LLMChatContextProps,
} from "@/krate/llmchat/component/session";
import { TID } from "@/lib/id_util";
import { useLLMChatStore } from "@/krate/llmchat/store";
import { llmchatSessionRecordFetch } from "@/krate/llmchat/service";
import Fullscreen from "./fullscreen";

const LLMChatChnot = ({
  otid,
  fullscreen,
  readonly,
  onPostSave,
  onSetFullscreen,
}: RichPropProps) => {
  const pidRef = createRef<Set<TID>>();

  const [props, setProps] = useState<LLMChatContextProps | undefined>(
    undefined,
  );
  const { refreshTemplates } = useLLMChatStore();

  useEffect(() => {
    refreshTemplates();
  }, []);

  // Used to load from database.
  useEffect(() => {
    (async () => {
      if (otid) {
        const rsp = await llmchatSessionRecordFetch({
          session_otid: otid,
        });
        if (rsp.session) {
          setProps(() => {
            const pids = new Set<TID>();

            if (rsp.session) {
              rsp.records.forEach((r) => {
                pids.add(r.otid);
              });
              pids.add(rsp.session.otid);
            }
            pidRef.current = pids;
            return {
              sessionOtid: otid,
              records: rsp.records,
              session: rsp.session,
              persistedIds: pidRef,
            };
          });
        } else {
          setProps({
            sessionOtid: otid,
            persistedIds: pidRef,
          });
        }
      }
    })();
  }, [otid]);
  console.log("props", props);

  return props ? (
    <LLMChatEditorProvider props={props}>
      {fullscreen ? (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          <SessionContainer
            onPostSave={(_) => {
              onPostSave({
                saveState: SaveState.Saved,
              });
            }}
            readonly={false}
          />
        </Fullscreen>
      ) : (
        <div className="m-0 p-0">
          {readonly ? (
            <SessionContainer readonly={true} />
          ) : (
            <SessionContainer
              readonly={false}
              onPostSave={(_) => {
                onPostSave({
                  saveState: SaveState.Saved,
                });
              }}
            />
          )}
        </div>
      )}
    </LLMChatEditorProvider>
  ) : (
    <div className="w-full h-full">
      <LLMChatTemplateList
        onSelectTemplate={(template: LLMChatTemplate) => {
          if (onSetFullscreen) {
            onSetFullscreen(true);
          }

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
                sessionOtid: otid,
                persistedIds: ref,
                template: template,
              };
              return props;
            }
          });
        }}
        onNew={() => {
          if (onSetFullscreen) {
            onSetFullscreen(true);
          }
          setProps({
            showTemplateForm: true,
            sessionOtid: otid,
            persistedIds: pidRef,
          });
        }}
      />
    </div>
  );
};

export default LLMChatChnot;
