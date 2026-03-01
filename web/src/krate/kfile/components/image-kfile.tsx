import { useState } from "react";
import Icon from "@/common/component/icon";
import RelativeTime from "@/common/component/relative-time";
import { Button } from "@/common/component/ui/button";
import { getResouceDownloadUrl } from "@/krate/kfile/service";
import { humanFileSize } from "@/lib/unit-utils";
import type { KFileMeta } from "../po";
import { handleDownloadUrl } from "./download";

export const isImageFile = (contentType?: string) => {
  if (contentType?.startsWith("image/")) return true;
  return false;
};

const ImageDetails = ({ kfile }: { kfile: KFileMeta }) => {
  return (
    <div className="grid grid-cols-2 gap-2 text-gray-600 dark:text-gray-300 text-sm font-medium p-3 bg-white/90 dark:bg-gray-800/90 rounded-lg shadow-lg min-w-[200px]">
      <div className="text-left font-semibold">Name:</div>
      <div className="line-clamp-2 break-all" title={kfile.filename}>
        {kfile.filename}
      </div>
      <div className="text-left font-semibold">Size:</div>
      <div>{humanFileSize(kfile.filesize, false)}</div>
      <div className="text-left font-semibold">Date:</div>
      <div>
        <RelativeTime date={new Date(kfile.last_modified / 1000)} />
      </div>
      <div className="text-left font-semibold">Type:</div>
      <div>{kfile.content_type || "unknown"}</div>
    </div>
  );
};

export const ImageKFile = ({
  kfile,
  onReplace,
  onIconClick,
}: {
  kfile: KFileMeta;
  onReplace?: () => void;
  onIconClick?: () => void;
}) => {
  const [showDetails, setShowDetails] = useState(false);

  if (!isImageFile(kfile.content_type)) {
    return null;
  }

  const imageUrl = getResouceDownloadUrl(kfile);

  return (
    <div className="relative inline-block">
      <img
        src={imageUrl}
        alt={kfile.filename}
        className="max-w-full h-auto rounded-lg shadow-md"
      />

      <div className="absolute top-2 left-2 flex gap-2">
        {onReplace && (
          <Button
            size="sm"
            variant="secondary"
            className="bg-white/80 dark:bg-gray-800/80 hover:bg-white dark:hover:bg-gray-700"
            onClick={onReplace}
            title="Replace file"
          >
            <Icon.RefreshCwIcon className="w-4 h-4" />
          </Button>
        )}

        <Button
          size="sm"
          variant="secondary"
          className="bg-white/80 dark:bg-gray-800/80 hover:bg-white dark:hover:bg-gray-700"
          onClick={() => setShowDetails(!showDetails)}
          title="Show details"
        >
          <Icon.InfoIcon className="w-4 h-4" />
        </Button>

        <Button
          size="sm"
          variant="secondary"
          className="bg-white/80 dark:bg-gray-800/80 hover:bg-white dark:hover:bg-gray-700"
          onClick={onIconClick || (() => handleDownloadUrl(imageUrl))}
          title="Download"
        >
          <Icon.DownloadIcon className="w-4 h-4" />
        </Button>
      </div>

      {showDetails && (
        <div className="absolute top-12 left-2 z-10">
          <ImageDetails kfile={kfile} />
        </div>
      )}
    </div>
  );
};
