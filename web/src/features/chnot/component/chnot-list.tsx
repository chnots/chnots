import { useEffect, useState } from "react";

import ChnotListItem from "./chnot-list-item";
import { useChnotStore } from "@/store/chnot/store";
import KPageList from "@/common/component/kpagelist";
import KButton from "@/common/component/kbutton";
import Icon from "@/common/component/icon";
import { chnotTagNames } from "@/store/chnot/service";
import ChnotTagListItem from "./chnot-tag-list-item";

const TagPath = () => {
  const { setListViewType, listViewType } = useChnotStore();
  if (listViewType.kind !== "tagtree") {
    return <></>
  }

  const tagKind = listViewType.tagkind;
  const tagPath = listViewType.tagpath;
  const parts = tagPath.length > 0 ? tagPath.split('/') : [];

  const segments: string[] = [];
  if (parts.length > 0) {
    segments.push(parts[0])
    for (let i = 1; i < parts.length; i++) {
      segments.push(`${segments[i - 1]}/${parts[i]}`);
    }
    parts[0] = parts[0].replace(RegExp("#"), "");
  }

  return <div className="w-full flex flex-row space-x-1">
    <div className="pl-2" />
    <KButton>{
      tagKind === "children"
        ? <Icon.WheatOff onClick={() => setListViewType({ ...listViewType, tagkind: "descendants" })} className="w-4" />
        : <Icon.Wheat onClick={() => setListViewType({ ...listViewType, tagkind: "children" })} className="w-4" />
    }
    </KButton>
    <div className="py-2 hover:cursor-pointer space-x-1" key={"#root"} ><span onClick={() => {
      setListViewType({ ...listViewType, tagpath: "" })
    }} className="underline">#</span></div>
    {parts.map((layer, index) => {
      return <div className="py-2 hover:cursor-pointer space-x-1" key={segments[index]} ><span onClick={() => {
        setListViewType({ ...listViewType, tagpath: segments[index] })
      }} className="underline">{layer}</span><span>/</span></div>
    })}
  </div>
}

const ChnotList = () => {
  const {
    fetchMoreChnots,
    refreshChnots,
    isFetchingNextPage,
    hasNextPage,
    chnotMapByMetaId,
    changeKeyword,
    listViewType,
    setListViewType
  } = useChnotStore();

  const [keyword, setKeyword] = useState<string>();
  const [tagList, setTagList] = useState<string[]>();

  useEffect(() => {
    changeKeyword(keyword);
  }, [keyword])

  useEffect(() => {
    if (listViewType.kind === "tagtree") {
      chnotTagNames({
        start_index: 0,
        page_size: 9999,
        query_type: listViewType,
        query: keyword
      }).then((rsp) => {
        setTagList(rsp.data)
      });
    }
    refreshChnots()
  }, [listViewType, setTagList, keyword]);

  return (
    <>
      <div className="flex flex-row">
        <KButton onClick={() => {
          if (listViewType.kind !== "timeline") {
            setListViewType({ kind: "timeline" })
          } else {
            setListViewType({ kind: "tagtree", tagkind: "children", tagpath: "" })
          }
        }}>
          {listViewType.kind === "timeline" ? <Icon.Inbox /> : <Icon.Folder />}
        </KButton>
        <div className="w-full p-2 bg-transparent rounded">
          <input
            type="text"
            className="bg-transparent w-full h-full outline-none border-b"
            placeholder="Search something"
            onChange={(value) => setKeyword(value.target.value)}
          />
        </div>
      </div>
      <TagPath />
      <div className="overflow-auto h-full overflow-x-hidden overflow-y-auto">
        {
          tagList && listViewType.kind === "tagtree" &&
          <ul className="m-0 grid gap-2 pt-2 pr-1 pb-1 pl-2">
            {tagList.map((tagpath) =>
              <ChnotTagListItem key={tagpath} tag={tagpath} handleClick={() => {
                if (listViewType.kind === "tagtree") {
                  setListViewType({ ...listViewType, tagpath })
                }
              }} />
            )}
          </ul>
        }
        <KPageList
          onFetchMore={fetchMoreChnots}
          isFetchingNextPage={isFetchingNextPage}
          hasNextPage={hasNextPage}
        >
          {[...chnotMapByMetaId.values()].map((chnot) => (
            <ChnotListItem chnot={chnot} key={chnot.record.id} />
          ))}
        </KPageList>
      </div>
    </>
  );
}

export default ChnotList;
