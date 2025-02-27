import { v4 as uuid } from "uuid";
import { useCallback, useRef, useState } from "react";
import { ChnotOverwriteReq } from "@/store/chnot";
import { useChnotStore } from "@/store/chnot";
import Icon from "@/common/component/icon";
import { CodeMirrorEditorMemo } from "@/common/component/codemirror-md-editor";
import useDebounce from "@/hooks/use-debounce";
import { useNamespaceStore } from "@/store/namespace";
import { NamespaceIcon } from "@/common/component/namespace-select";
import { Menu, MenuItem, MenuButton } from "@szhsin/react-menu";

enum RequestState {
  Saved,
  Requesting,
  Error,
}

interface ChnotEditState {
  isUploadingResource: boolean;
  requestState: RequestState;
  isComposing: boolean;
}

export const ChnotMarkdownEditor = () => {
  console.log("Frame changed");

  const { namespaces, currentNamespace } = useNamespaceStore();
  const {
    currentChnotIndex,
    chnotMap,
    setCurrentChnot,
    overwriteChnot,
    queryChnot,
    updateChnot,
    validateChnotCache,
  } = useChnotStore();

  const currentChnot = currentChnotIndex
    ? chnotMap.get(currentChnotIndex)
    : undefined;

  const [editState, setEditState] = useState<ChnotEditState>({
    isUploadingResource: false,
    requestState: RequestState.Saved,
    isComposing: false,
  });

  const saveContent = useCallback(
    async (metaId: string, content?: string) => {
      setEditState((state) => {
        return {
          ...state,
          requestState: RequestState.Requesting,
        };
      });

      const req: ChnotOverwriteReq = {
        chnot: {
          id: uuid(),
          content: content ?? "",
          insert_time: new Date(),
          meta_id: metaId,
        },
        kind: "mdwt",
      };
      let requestState;
      try {
        const rsp = await overwriteChnot(req, true);
        setCurrentChnot(rsp.chnot);
        requestState = RequestState.Saved;
      } catch {
        requestState = RequestState.Error;
      }
      setEditState((state) => {
        return {
          ...state,
          requestState,
        };
      });
    },
    [setCurrentChnot, setEditState, editState]
  );

  const onChange = useDebounce((metaId: string, content: string) => {
    console.log("begin to save ", content);
    saveContent(metaId, content);
  }, 1000);

  const onChangeRef = useRef(onChange);
  const fetchContent = useCallback(
    async (id: string) => {
      const chnot = await queryChnot({ meta_id: id });
      return chnot?.record.content;
    },
    [queryChnot]
  );

  return (
    <>
      <div className="w-full p-1 flex space-x-2">
        <div className="text-xs">
          {editState.requestState === RequestState.Requesting ? (
            <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.Loader2 className="animate-spin h-5 w-5 mr-2" />
            </div>
          ) : editState.requestState === RequestState.Error ? (
            <div className="flex items-center text-red-600 transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.LucideMessageCircleQuestion className="h-5 w-5 mr-2" />
            </div>
          ) : (
            <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.CheckCircle className="h-5 w-5 mr-2" />
            </div>
          )}
        </div>
        {currentChnot && (
          <Menu
            menuButton={
              <MenuButton>
                <NamespaceIcon
                  name={currentChnot?.meta.namespace}
                ></NamespaceIcon>
              </MenuButton>
            }
            transition
          >
            {namespaces().map((e) => (
              <MenuItem
                key={e.name}
                onClick={() => {
                  if (currentChnot) {
                    updateChnot({
                      meta_id: currentChnot.meta.id,
                      update_time: false,
                      namespace: e.name,
                    }).then((_) => {
                      if (e.name !== currentNamespace.name) {
                        validateChnotCache([currentChnot.meta.id]);
                      }
                    });
                  }
                }}
              >
                {e.name}
              </MenuItem>
            ))}
          </Menu>
        )}
        <div>{currentChnot?.meta.insert_time.toDateString()}</div>
        <span>~</span>
        <div>{currentChnot?.record.insert_time.toDateString()}</div>
      </div>
      <div className="w-full h-98/100">
        <CodeMirrorEditorMemo
          onChange={onChangeRef}
          className="border kborder shadow-lg my-3"
          id={currentChnot?.meta.id ?? uuid()}
          fetchDefaultValue={fetchContent}
        />
      </div>
    </>
  );
};
