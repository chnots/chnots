import { useEffect, useState } from "react";

import KPageList from "@/common/component/kpagelist";
import ChnotTagListItem from "./chnot-tag-list-item";
import { chnotTagNames } from "@/store/chnot/service";
import KInput from "@/common/component/kinput";
import { useChnotStore } from "@/store/chnot/store";
import KButton from "@/common/component/kbutton";
import Icon from "@/common/component/icon";

function ChnotTagList() {
  const { setTagKeyword, setCurrentChnot, tagKeyword, refreshChnots } = useChnotStore();

  const [keyword, setKeyword] = useState<string>();
  const [isFetchingNextPage, setIsFetchingNextPage] = useState<boolean>(false);
  const [hasNextPage, setHasNextPage] = useState<boolean>(true);
  const [tagList, setTagList] = useState<string[]>([]);
  const [startIndex, setStartIndex] = useState<number>(0);

  useEffect(() => {
    (async () => {
      if (!isFetchingNextPage && hasNextPage) {
        setIsFetchingNextPage(true);
        const tags = await chnotTagNames({
          query: keyword,
          start_index: startIndex,
          page_size: 20,
        });
        if (tags.data.length < 20) {
          setHasNextPage(false);
        }
        setStartIndex(startIndex + tags.data.length);
        setTagList((prev) => [...prev.slice(0, startIndex), ...tags.data]);
        setIsFetchingNextPage(false);
      }
    })();
  }, [keyword, isFetchingNextPage, hasNextPage, startIndex]);

  useEffect(() => {
    refreshChnots();
    setCurrentChnot()
  }, [tagKeyword]);

  return (
    <div className="max-h-full">
      <div className="flex flex-row">
        <KButton onClick={() => { setKeyword(undefined); setTagKeyword(undefined) }}>
          <Icon.CircleX />
        </KButton>
        <KInput
          onChange={(input) => {
            setKeyword(input);
            setStartIndex(0);
            setHasNextPage(true);
          }}
        />
      </div>
      <KPageList
        onFetchMore={() => { }}
        isFetchingNextPage={isFetchingNextPage}
        hasNextPage={hasNextPage}
      >
        {tagList.map((tag) => (
          <ChnotTagListItem
            tag={tag}
            key={tag}
            focused={tagKeyword === tag}
            handleClick={() => setTagKeyword(tag)}
          />
        ))}
      </KPageList>
    </div>
  );
}

export default ChnotTagList;
