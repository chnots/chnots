import { openUrl } from "@tauri-apps/plugin-opener";
import { isTauri } from "@/lib/request";
import type { KFileMeta } from "../po";
import { getResouceDownloadUrl } from "../service";

export const handleDownloadKfile = (kfile: KFileMeta) => {
  return handleDownloadUrl(getResouceDownloadUrl(kfile));
};

export const handleDownloadUrl = (url: string) => {
  if (isTauri) {
    openUrl(url);
  } else {
    window.open(url, "_blank");
  }
};
