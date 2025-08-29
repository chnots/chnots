import { createContext, useContext, useState } from "react";
import { ChnotMeta, ChnotThreadMeta } from "../../po";
import ChnotBlocks from "./thread";
import { TID } from "@/lib/id_util";
import LoadingPage from "@/common/pages/loading-page";
import { SaveState } from "@/common/types";
import { createStore, StoreApi, useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";

interface ChnotRopeProps {
  chnotMetaOtid: TID;
  readonly: boolean;
}

interface ChnotRopeState extends ChnotRopeProps {
  isUploading: boolean;
  saveState: SaveState;
  isComposing: boolean;
}

function createChnotStore(props: ChnotRopeProps) {
  return createStore<ChnotRopeState>()((set) => ({
    ...props,
    isUploading: false,
    saveState: SaveState.Saved,
    isComposing: false,
    isMobile: false,
  }));
}

const ChnotRopeContext = createContext<StoreApi<ChnotRopeState> | null>(null);

export function useChnotRopeStore<T>(selector: (state: ChnotRopeState) => T) {
  const store = useContext(ChnotRopeContext);

  return useStore(
    store!,
    useShallow((store) => {
      return selector(store);
    }),
  );
}

function ChnotRopeProvider({
  props,
  children,
}: {
  props: ChnotRopeProps;
  children: React.ReactNode;
}) {
  const [store] = useState<StoreApi<ChnotRopeState>>(createChnotStore(props));

  return store ? (
    <ChnotRopeContext.Provider value={store}>
      {children}
    </ChnotRopeContext.Provider>
  ) : (
    <LoadingPage />
  );
}

const ChnotRope = ({ meta }: { meta: ChnotThreadMeta }) => {
  return (
    <ChnotRopeProvider
      props={{
        chnotMetaOtid: meta.otid,
        readonly: false,
      }}
    >
      <div className="flex flex-col w-full items-center overflow-y-auto">
        <ChnotBlocks threadOtid={meta.otid} />
      </div>
    </ChnotRopeProvider>
  );
};

export default ChnotRope;
