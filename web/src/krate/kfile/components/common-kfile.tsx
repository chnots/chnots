import {
  getResouceDownloadUrl,
  kfileMetaFetch,
  kfileUpload,
} from "@/krate/kfile/service";
import { genUID, TID } from "@/lib/id_util";
import { useCallback, useEffect, useState } from "react";

import { Button, Button as KButton } from "@/common/component/ui/button";
import RelativeTime from "@/common/component/relative-time";
import { humanFileSize } from "@/lib/unit-utils";
import FileNameToIcon from "./filename-to-icon";
import { KFileMeta } from "../po";
import { isTauri } from "@/lib/request";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useIsMobile } from "@/hooks/use-mobile";

type FileLike = {
  name: string;
  size: number;
  modified: Date;
};

const FileInfo = ({ file }: { file: FileLike }) => {
  return (
    <div className="grid grid-cols-2 gap-2 text-gray-600 dark:text-gray-300 text-sm font-medium">
      <div className="text-left font-semibold">Name:</div>
      <div className="line-clamp-2 break-all" title={file.name}>
        {file.name}
      </div>
      <div className="text-left font-semibold">Size:</div>
      <div>{humanFileSize(file.size, false)}</div>
      <div className="text-left font-semibold">Date:</div>
      <div>
        <RelativeTime date={new Date(file.modified)} />
      </div>
    </div>
  );
};

// inspired by https://github.com/AarambhDevHub/frontend-file-Chunks/blob/main/app/page.tsx
export const CommonKFile = ({
  otid,
  onPostSave,
}: {
  otid: TID;
  onPostSave?: (r: KFileMeta) => void;
}) => {
  const [progress, setProgress] = useState(0);
  const [uploadFile, setUploadFile] = useState<File | undefined>(undefined);
  const [kfile, setKFile] = useState<KFileMeta | undefined>(undefined);
  const [isDragging, setIsDragging] = useState(false);
  const isMobile = useIsMobile();

  console.log("kid: ", otid);
  useEffect(() => {
    if (otid) {
      kfileMetaFetch({ req_id: { Otid: otid } }).then(({ meta }) => {
        setKFile(meta);
      });
    }
  }, [otid]);
  const uploadFileInChunks = async () => {
    if (!uploadFile) {
      alert("Please select a file to upload");
      return;
    }

    const chunkSize = 1 * 1024 * 1024; // 1 MB
    const totalChunks = Math.ceil(uploadFile.size / chunkSize);
    let currentChunk = Number(localStorage.getItem(uploadFile.name)) || 0;
    const uploadId = genUID();

    setProgress(0);

    while (currentChunk < totalChunks) {
      const start = currentChunk * chunkSize;
      const end = Math.min(start + chunkSize, uploadFile.size);
      const chunk = uploadFile.slice(start, end);

      try {
        const { kfile } = await kfileUpload({
          upload_id: uploadId,
          chunk,
          filename: uploadFile.name,
          chunk_no: currentChunk,
          total_chunks: totalChunks,
          meta_id: uploadId,
          content_type: uploadFile.type,
          filesize: uploadFile.size,
          last_modified: uploadFile.lastModified,
          otid: otid,
        });

        if (kfile) {
          if (onPostSave) {
            onPostSave(kfile);
          }
          setKFile(kfile);
          setUploadFile(undefined);
        }

        currentChunk++;
        localStorage.setItem(uploadFile.name, currentChunk.toString());
        setProgress(parseInt(((currentChunk / totalChunks) * 100).toFixed(0)));
      } catch (error) {
        console.error(error);
        alert("An error occurred during upload.");
        return;
      }
    }

    localStorage.removeItem(uploadFile.name);
  };

  const handleFileChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      if (file) {
        setUploadFile(file);
        setProgress(0);
      }
    },
    [],
  );

  const handleDragOver = useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      event.preventDefault();
      setIsDragging(true);
    },
    [],
  );

  const handleDragLeave = useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      event.preventDefault();
      setIsDragging(false);
    },
    [],
  );

  const handleDrop = useCallback((event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    setIsDragging(false);
    const file = event.dataTransfer.files?.[0];
    if (file) {
      setUploadFile(file);
      setProgress(0);
    }
  }, []);

  return (
    <div
      className={`flex ${isMobile ? "flex-col space-y-6" : "flex-row space-x-6"}  m-4 w-full pr-10 items-center`}
    >
      <div className="flex flexcol justify-center align-middle h-full">
        <div
          className={`
              relative group border-1 border-dashed rounded-xl p-8 text-center
              transition-all duration-300 ease-out cursor-pointer 
            ${
              isDragging
                ? "border-blue-500 bg-blue-50/50 dark:bg-blue-900/20"
                : "border-gray-200 dark:border-gray-700 hover:border-blue-400 dark:hover:border-blue-500"
            }
          `}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
          role="button"
          tabIndex={0}
          aria-label="File upload area"
          onClick={() => document.getElementById("file-input")?.click()}
          onKeyDown={(e) =>
            e.key === "Enter" && document.getElementById("file-input")?.click()
          }
        >
          <input
            id="file-input"
            type="file"
            onChange={handleFileChange}
            className="hidden"
            aria-describedby="file-input-help"
          />
          <FileNameToIcon
            filename={kfile?.filename ?? ""}
            className="w-14 h-14"
            strokeWidth={1}
          />
        </div>
      </div>

      {uploadFile && (
        <div className="space-y-2 max-w-1/2 flex flex-col border p-4 rounded-2xl">
          <FileInfo
            file={{
              name: uploadFile.name,
              size: uploadFile.size,
              modified: new Date(uploadFile.lastModified),
            }}
          />
          {progress < 100 && (
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Upload Progress
                </span>
                <span className="text-sm font-mono text-blue-600 dark:text-blue-400">
                  {progress}%
                </span>
              </div>

              <div className="relative">
                <div className="h-2.5 bg-gray-100 dark:bg-gray-800 rounded-full overflow-hidden">
                  <div
                    className="h-full bg-blue-600 dark:bg-blue-500 rounded-full transition-all duration-300 ease-out"
                    style={{ width: `${progress}%` }}
                    role="progressbar"
                    aria-valuenow={progress}
                    aria-valuemin={0}
                    aria-valuemax={100}
                  >
                    <div className="absolute inset-0 animate-progress-stripes bg-[length:1.5rem_1.5rem] bg-repeat" />
                  </div>
                </div>
              </div>
            </div>
          )}

          <div className=" flex w-full justify-center">
            <KButton
              onClick={uploadFileInChunks}
              disabled={!uploadFile}
              className={
                "max-w-20 w-full py-3.5 px-6 rounded-lg align-center justify-center border kc-basic-with-bdr"
              }
            >
              {progress > 0 && progress < 100 ? "Uploading..." : "Upload"}
            </KButton>
          </div>
        </div>
      )}

      {kfile && (
        <div className="space-y-2 max-w-1/2 flex flex-row p-4">
          <div className="text-gray-600 dark:text-gray-300 text-sm font-medium">
            <FileInfo
              file={{
                name: kfile.filename,
                size: kfile.filesize,
                modified: new Date(kfile.last_modified / 1000),
              }}
            />
            <div className="flex p-5 justify-center align-middle items-center ">
              {isTauri ? (
                <Button
                  onClick={() => {
                    openUrl(getResouceDownloadUrl(kfile));
                  }}
                >
                  Export
                </Button>
              ) : (
                <Button asChild>
                  <a href={getResouceDownloadUrl(kfile)}>Download</a>
                </Button>
              )}
            </div>
          </div>
        </div>
      )}

      <style>{`
      @keyframes progress-stripes {
      from {
      background-position: 0 0;
      }
      to {
      background-position: 1.5rem 0;
      }
      }
      .animate-progress-stripes {
      background-image: linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.15) 25%,
      transparent 25%,
      transparent 50%,
      rgba(255, 255, 255, 0.15) 50%,
      rgba(255, 255, 255, 0.15) 75%,
      transparent 75%,
      transparent
      );
      animation: progress-stripes 1s linear infinite;
      }
      `}</style>
    </div>
  );
};
