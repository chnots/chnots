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
    return <></>;
  }

  const { tagkind, tagpath } = listViewType;
  const parts = tagpath.length > 0 ? tagpath.split("/") : [];

  const segments: string[] = [];
  if (parts.length > 0) {
    segments.push(parts[0]);
    for (let i = 1; i < parts.length; i++) {
      segments.push(`${segments[i - 1]}/${parts[i]}`);
    }
    parts[0] = parts[0].replace(RegExp("#"), "");
  }

  return (
    <div className="w-full flex flex-row space-x-1 items-center">
      <div className="pl-2" />
      <KButton className="py-1 px-2">
        {tagkind === "children" ? (
          <Icon.WheatOff
            onClick={() =>
              setListViewType({ ...listViewType, tagkind: "descendants" })
            }
            className="w-4"
          />
        ) : (
          <Icon.Wheat
            onClick={() =>
              setListViewType({ ...listViewType, tagkind: "children" })
            }
            className="w-4"
          />
        )}
      </KButton>
      <KButton
        className="p-1 underline"
        key={"#root"}
        onClick={() => {
          setListViewType({ ...listViewType, tagpath: "" });
        }}
      >
        #
      </KButton>
      {parts.map((layer, index) => {
        return (
          <>
            <KButton
              className="p-1"
              key={segments[index]}
              onClick={() => {
                setListViewType({ ...listViewType, tagpath: segments[index] });
              }}
            >
              <span className="underline">{layer}</span>
            </KButton>
            <span>/</span>
          </>
        );
      })}
    </div>
  );
};

const ChnotList = () => {
  const {
    fetchMoreChnots,
    refreshChnots,
    isFetchingNextPage,
    chnotMapByMetaId,
    changeKeyword,
    listViewType,
    setListViewType,
  } = useChnotStore();

  const [keyword, setKeyword] = useState<string>();
  const [tagList, setTagList] = useState<string[]>();

  useEffect(() => {
    changeKeyword(keyword);
  }, [keyword]);

  useEffect(() => {
    if (listViewType.kind === "tagtree") {
      chnotTagNames({
        start_index: 0,
        page_size: 9999,
        tag_tree: listViewType,
        query: keyword,
      }).then((rsp) => {
        setTagList(rsp.data);
      });
    }
    refreshChnots();
  }, [listViewType, setTagList, keyword, refreshChnots]);

  return (
    <>
      <div className="flex flex-row">
        <KButton
          onClick={() => {
            if (listViewType.kind !== "timeline") {
              setListViewType({ kind: "timeline" });
            } else {
              setListViewType({
                kind: "tagtree",
                tagkind: "children",
                tagpath: "",
              });
            }
          }}
          className="p-2"
        >
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
        {tagList && listViewType.kind === "tagtree" && (
          <ul className="m-0 grid gap-2 pt-2 pr-1 pb-1 pl-2">
            {tagList.map((tagpath) => (
              <ChnotTagListItem
                key={tagpath}
                tag={tagpath}
                handleClick={() => {
                  if (listViewType.kind === "tagtree") {
                    setListViewType({ ...listViewType, tagpath });
                  }
                }}
              />
            ))}
          </ul>
        )}
        <KPageList
          onFetchMore={fetchMoreChnots}
          isFetchingNextPage={isFetchingNextPage}
          hasNextPage={chnotMapByMetaId.hasNextPage}
        >
          {[...chnotMapByMetaId.dbCache.values()].map((chnot) => (
            <ChnotListItem chnot={chnot} key={chnot.record.id} />
          ))}
        </KPageList>
      </div>
    </>
  );
};

export default ChnotList;
