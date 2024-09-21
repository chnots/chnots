import { useNamespaceStore } from "@/store/namespace";

const useCurrentnamespace = () => {
  return useNamespaceStore().current;
};

export default useCurrentnamespace;
