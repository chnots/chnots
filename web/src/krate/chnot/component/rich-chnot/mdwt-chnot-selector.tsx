import { useCallback, useEffect, useRef, useState } from "react";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/common/component/ui/command";

import type { TID } from "@/lib/id_util";
import type { TimeoutType } from "@/lib/types";
import type { ChnotSearchReq, ChnotSearchRspData } from "../../dto";
import type { ChnotKind } from "../../po";
import { chnotSearch } from "../../service";
import { ChnotKindIcon } from "../kind-icon";

const MdwtChnotSelector = ({
  onSelect,
  excludeList,
}: {
  onSelect: (otid: TID, kind: ChnotKind) => void;
  excludeList: { otid: TID }[];
}) => {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<ChnotSearchRspData[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const timeoutRef = useRef<TimeoutType | null>(null);
  const excludeOtids = useRef<Set<TID>>(
    new Set(excludeList.map((e) => e.otid)),
  );

  const handleSearch = useCallback(async (searchQuery: string) => {
    if (!searchQuery.trim()) {
      setResults([]);
      return;
    }

    setIsLoading(true);
    try {
      const req: ChnotSearchReq = {
        query: searchQuery,
        kinds: [],
        start_index: 0,
        page_size: 20,
      };

      const response = await chnotSearch(req);

      setResults(
        response.data.filter((e) => !excludeOtids.current.has(e.meta.otid)),
      );
    } catch (error) {
      console.error("Error searching chnots:", error);
      setResults([]);
    } finally {
      setIsLoading(false);
    }
  }, []);

  const handleInputChange = useCallback(
    (value: string) => {
      setQuery(value);

      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }

      timeoutRef.current = setTimeout(() => {
        handleSearch(value);
      }, 300);
    },
    [handleSearch],
  );

  const handleSelect = useCallback(
    (selectedOtid: TID, kind: ChnotKind) => {
      onSelect(selectedOtid, kind);
    },
    [onSelect],
  );

  useEffect(() => {
    console.log("result changed", results);

    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, [results]);

  return (
    <div className="w-full px-10">
      <Command shouldFilter={false}>
        <CommandInput
          placeholder="Search chnots..."
          value={query}
          onValueChange={handleInputChange}
          className="w-full"
        />
        <CommandList>
          <CommandGroup>
            {isLoading ? (
              <CommandEmpty>Loading...</CommandEmpty>
            ) : results.length > 0 ? (
              results.map((result) => (
                <CommandItem
                  key={result.meta.otid}
                  onSelect={() => {
                    console.warn("select, ", result.meta.otid);
                    handleSelect(result.meta.otid, result.meta.kind);
                  }}
                >
                  <div className="flex items-center gap-2">
                    <ChnotKindIcon kind={result.meta.kind} />
                    <span className="text-sm text-muted-foreground">
                      {result.meta.kspace}
                    </span>
                    <span className="font-medium">
                      {result.title || `${result.meta.otid}`}
                    </span>
                  </div>
                </CommandItem>
              ))
            ) : query ? (
              <CommandEmpty>No chnots found.</CommandEmpty>
            ) : (
              <CommandEmpty>Type to search chnots...</CommandEmpty>
            )}
          </CommandGroup>
        </CommandList>
      </Command>
    </div>
  );
};

export default MdwtChnotSelector;
