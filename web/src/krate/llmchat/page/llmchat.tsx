import SessionList from "@/krate/llmchat/component/session-list";
import { useKSpaceStore } from "@/krate/kspace/store";
import { useEffect, useState } from "react";
import { useLLMChatStore } from "@/krate/llmchat/store";
import { v4 as uuid } from "uuid";
import { genTID, TID } from "@/lib/id_util";
import { useCommonStore } from "@/common/store";
import SessionContainer, {
  LLMChatEditorProvider,
} from "../component/llm-chat-session";

const LLMChatPage = () => {
  const [otid, setOtid] = useState(genTID());
  return (
    <LLMChatEditorProvider
      props={{
        sessionOtid: otid,
        viewMode: false,
      }}
    >
      <SessionContainer />
    </LLMChatEditorProvider>
  );
};

export default LLMChatPage;
