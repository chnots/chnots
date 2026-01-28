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
import { ChnotKind } from "../../po";
import { chnotSingleSearch } from "../../service";

const MdwtChnotSelector = ({ onSelect }: { onSelect: (otid: TID) => void }) => {
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
        kinds: [ChnotKind.MDWT],
        with_archive: false,
        start_index: 0,
        page_size: 20,
      };

      const response = await chnotSingleSearch(req);
      console.log("data: ", response.data);
      setResults(response.data);
    } catch (error) {
      console.error("Error searching MDWT chnots:", error);
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
    (selectedOtid: TID) => {
      onSelect(selectedOtid);
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
    <div className="w-full max-w-md">
      <Command shouldFilter={false}>
        <CommandInput
          placeholder="Search MDWT chnots..."
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
                    handleSelect(result.meta.otid);
                  }}
                >
                  <div className="flex items-center gap-2">
                    <span className="text-sm text-muted-foreground">
                      {result.meta.kspace}
                    </span>
                    <span className="font-medium">
                      {result.title || `MDWT ${result.meta.otid}`}
                    </span>
                  </div>
                </CommandItem>
              ))
            ) : query ? (
              <CommandEmpty>No chnots found.</CommandEmpty>
            ) : (
              <CommandEmpty>Type to search MDWT chnots...</CommandEmpty>
            )}
          </CommandGroup>
        </CommandList>
      </Command>
    </div>
  );
};

export default MdwtChnotSelector;
