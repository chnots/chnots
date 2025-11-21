import React, { useCallback, useEffect, useRef, useState } from "react";

import {
	chnotMetaCommit,
	chnotThreadMetaFetch,
	chnotThreadMetaOverwrite,
	chnotThreadOrderCommit,
} from "../../service";
import RichMdwt from "../rich-chnot/rich-mdwt";

import { ChnotKind, type ChnotThreadMeta } from "../../po";
import type { PostSaveArg } from "../rich-chnot/rich-chnot";
import LoadingPage from "@/common/pages/loading-page";
import { SaveState } from "@/common/types";
import type { MdwtRecord } from "@/krate/mdwt/po";
import { mdwtRecordList } from "@/krate/mdwt/service";
import { arraysAreEqual } from "@/lib/col-util";
import { genTID, type TID } from "@/lib/id_util";
import type { ChnotThreadMetaCommitReq } from "../../dto";
import useDebugChanged from "@/hooks/use-debug-changed";

enum ChnotState {
	Initialized,
	Saved,
}

/**
 * This is the main component for the `Chnots` app.
 *
 * - Initial State
 *   - If the first `chnot` is `mdwt` type, just use it as the first `chnot`, which is called as `HEAD_CHNOT`.
 *   - If the first `chnot` is any other type(FOO), make the first chnot is `mdwt`, and create a new chnot, which type is FOO.
 *     - When we save the FOO chnot, initialize the HEAD_CHNOT and save it
 * - Maintain State
 *   - Just edit and save that chnot.
 */
const ChnotThreadBody = ({ threadMeta }: { threadMeta: ChnotThreadMeta }) => {
	const savedChnotMetaRef = useRef<Map<TID, ChnotState>>(new Map());
	const savedChnotOrdersRef = useRef<TID[]>([]);
	const savedChnotThreadMetaRef = useRef<ChnotThreadMeta>(undefined);
	const [chnotOrders, setChnotOrders] = useState<TID[]>([]);
	const [mdwtMap, setMdwtMap] = useState<Record<string, MdwtRecord>>({});
	const [loading, setLoading] = useState<boolean>(true);

	useDebugChanged(threadMeta, "threadMeta");
	useDebugChanged(mdwtMap, "mdwtMap");
	useDebugChanged(chnotOrders, "chnotOrders");

	useEffect(() => {
		(async () => {
			try {
				const rsp = await chnotThreadMetaFetch({
					thread_otid: threadMeta.otid,
				});

				if (rsp.thread_meta) {
					savedChnotThreadMetaRef.current = rsp.thread_meta;
				}

				if (rsp.chnot_meta_sorted.length === 0) {
					setChnotOrders([genTID()]);
				} else {
					const chnotOtids = rsp.chnot_meta_sorted.map((cm) => cm.otid);

					savedChnotOrdersRef.current = chnotOtids;
					savedChnotMetaRef.current = new Map(
						chnotOtids.map((obj) => [obj, ChnotState.Saved]),
					);

					const mdwtMap = await mdwtRecordList({
						mdwt_otids: chnotOtids,
					});
					setChnotOrders([...chnotOtids, genTID()]);
					setMdwtMap(mdwtMap.mdwt_map);
				}
			} finally {
				setLoading(false);
			}
		})();
	}, [threadMeta]);

	const handlePostSaveOnChnot = useCallback(
		async (arg: PostSaveArg) => {
			if (!savedChnotThreadMetaRef.current) {
				const req: ChnotThreadMetaCommitReq = {
					meta_otid: threadMeta.otid,
					kspace: threadMeta.kspace,
				};
				const rsp = await chnotThreadMetaOverwrite(req);
				savedChnotThreadMetaRef.current = rsp.meta;
			}

			console.log("chnotMeta", savedChnotMetaRef, arg.otid);
			if (savedChnotMetaRef.current.get(arg.otid) !== ChnotState.Saved) {
				await chnotMetaCommit({
					metas: [
						{
							otid: arg.otid,
							kind: ChnotKind.MDWT,
							kspace: threadMeta.kspace,
						},
					],
				});
				savedChnotMetaRef.current.set(arg.otid, ChnotState.Saved);
			}

			const toSaveChnotOrders = chnotOrders.filter(
				(e) => savedChnotMetaRef.current.get(e) === ChnotState.Saved,
			);
			if (!arraysAreEqual(toSaveChnotOrders, savedChnotOrdersRef.current)) {
				await chnotThreadOrderCommit({
					thread_otid: threadMeta.otid,
					orders: toSaveChnotOrders.map((e) => {
						return { otid: e };
					}),
				});
				savedChnotOrdersRef.current = toSaveChnotOrders;
			}
		},
		[chnotOrders, threadMeta],
	);

	return (
		<div className="flex flex-col w-full items-center overflow-y-auto">
			{loading ? (
				<LoadingPage />
			) : (
				<div className="flex flex-col space-y-1 p-4 m-2 w-full max-w-4xl items-center">
					{chnotOrders.map((otid, _index) => {
						return (
							<RichMdwt
								key={otid}
								otid={otid}
								onPostSave={(arg: PostSaveArg) => {
									if (arg.saveState === SaveState.Saved) {
										handlePostSaveOnChnot(arg);
									}
								}}
								content={mdwtMap[otid]?.content ?? ""}
								onChanged={(): void => {
									const inited = savedChnotMetaRef.current;
									if (!inited.get(otid)) {
										inited.set(otid, ChnotState.Initialized);
									}

									if (inited.has(otid)) {
										const lastOtid = chnotOrders.at(chnotOrders.length - 1);
										if (lastOtid && inited.has(lastOtid)) {
											setChnotOrders((prev) => {
												return [...prev, genTID()];
											});
										}
									}
								}}
							/>
						);
					})}
				</div>
			)}
		</div>
	);
};

export const ChnotThreadBodyMemo = React.memo(ChnotThreadBody);

export default ChnotThreadBody;
