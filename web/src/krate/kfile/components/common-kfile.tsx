import {
  getResouceDownloadUrl,
  kfileQueryInfo,
  kfileUpload,
} from "@/krate/kfile/service";
import { genUID, genTID, TID } from "@/lib/id_util";
import { useCallback, useEffect, useState } from "react";

import { Button as KButton } from "@/common/component/ui/button";
import RelativeTime from "@/common/component/relative-time";
import { humanFileSize } from "@/lib/unit-utils";
import FileNameToIcon from "./filename-to-icon";
import { queryKKV } from "@/krate/kkv/service";
import { KFileMeta } from "../po";

type FileLike = {
  name: string;
  size: number;
  modified: Date;
};

const KFileIcon = ({ file }: { file?: FileLike }) => {
  return (
    <div className="flex flex-col items-center justify-center space-y-4">
      <div
        className={`
        p-3 rounded-2xl transition-transform duration-300
              ${
                file
                  ? "bg-green-100 dark:bg-green-900/30"
                  : "bg-blue-100 dark:bg-blue-900/30"
              }
            `}
      >
        <FileNameToIcon
          filename={file?.name ?? ""}
          className="w-50 h-50"
          strokeWidth={1}
        />
      </div>

      <div className="space-y-2">
        {file ? (
          <div className="text-gray-600 dark:text-gray-300 text-sm font-medium">
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
          </div>
        ) : (
          <p className="text-gray-600 dark:text-gray-300 text-sm font-medium">
            Drag and drop or browse files
          </p>
        )}
      </div>
    </div>
  );
};

// inspired by https://github.com/AarambhDevHub/frontend-file-Chunks/blob/main/app/page.tsx
export const CommonKFile = ({
  kindId,
  onAfterSave,
}: {
  kindId?: string;
  onAfterSave?: (r: KFileMeta) => void;
}) => {
  const [progress, setProgress] = useState(0);
  const [uploadFile, setUploadFile] = useState<File | undefined>(undefined);
  const [kfile, setKFile] = useState<KFileMeta | undefined>(undefined);
  const [isDragging, setIsDragging] = useState(false);

  useEffect(() => {
    if (kindId) {
      kfileQueryInfo({ meta_id: kindId.toString() }).then(({ meta }) => {
        setKFile(meta);
      });
    }
  }, [kindId]);
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
        });

        if (kfile) {
          if (onAfterSave) {
            onAfterSave(kfile);
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
    []
  );

  const handleDragOver = useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      event.preventDefault();
      setIsDragging(true);
    },
    []
  );

  const handleDragLeave = useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      event.preventDefault();
      setIsDragging(false);
    },
    []
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
    <div className="flex flex-row space-x-2 m-4">
      {kfile && (
        <div className="flex flex-col">
          <div className="max-w-xl w-full align-center justify-center flex">
            <div
              className={`
              relative group border-2 border-dashed rounded-xl p-8 text-center
              transition-all duration-300 ease-out cursor-pointer border-blue-500 hover:bg-accent/50`}
            >
              <KFileIcon
                file={{
                  name: kfile.filename,
                  size: kfile.filesize,
                  modified: new Date(kfile.last_modified),
                }}
              />
            </div>
          </div>
          <a
            className="flex w-full p-5 justify-center align-middle items-center"
            href={getResouceDownloadUrl(kfile)}
          >
            Download
          </a>
        </div>
      )}
      {(uploadFile || !kfile) && (
        <div className="max-w-xl w-full align-center justify-center flex">
          <div className="space-y-8">
            <div
              className={`
              relative group border-2 border-dashed rounded-xl p-8 text-center
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
                e.key === "Enter" &&
                document.getElementById("file-input")?.click()
              }
            >
              <input
                id="file-input"
                type="file"
                onChange={handleFileChange}
                className="hidden"
                aria-describedby="file-input-help"
              />
              <KFileIcon
                file={
                  uploadFile
                    ? {
                        name: uploadFile.name,
                        size: uploadFile.size,
                        modified: new Date(uploadFile.lastModified),
                      }
                    : undefined
                }
              />
            </div>

            {progress < 100 && (
              <div className="space-y-6">
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

                <KButton
                  onClick={uploadFileInChunks}
                  disabled={!uploadFile}
                  className={
                    "max-w-20 w-full py-3.5 px-6 rounded-lg align-center justify-center border kc-basic-with-bdr"
                  }
                >
                  {progress > 0 && progress < 100 ? (
                    <>Uploading...</>
                  ) : (
                    "Upload"
                  )}
                </KButton>
              </div>
            )}
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
