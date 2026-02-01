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
import type { ChnotSearchReq, ChnotSearchRspSingle } from "../../dto";
import { chnotSingleSearch } from "../../service";
import { ChnotKindIcon } from "../kind-icon";
import type { ChnotKind } from "../../po";

const MdwtChnotSelector = ({
  onSelect,
}: {
  onSelect: (otid: TID, kind: ChnotKind) => void;
}) => {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<ChnotSearchRspSingle[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

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
        with_archive: false,
        start_index: 0,
        page_size: 20,
      };

      const response = await chnotSingleSearch(req);
      console.log("data: ", response.data);
      setResults(response.data);
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
