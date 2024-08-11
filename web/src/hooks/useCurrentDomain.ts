import { useDomainStore } from "@/store/v1/domain";

const useCurrentDomain = () => {
  return useDomainStore().current;
};

export default useCurrentDomain;
