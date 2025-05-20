import { resourceUpload } from "@/store/resource/service";
import { genId } from "@/utils/id_util";
import { JSX, useCallback, useState } from "react";

import Icon from "@/common/component/icon";
import KButton from "@/common/component/kbutton";
import RelativeTime from "@/common/component/relative-time";
import { humanFileSize } from "@/utils/unit-utils";
import { Resource } from "@/store/resource/db";

const FileNameToIcon = ({
  filename,
  ...rest
}: {
  filename: string;
} & Icon.LucideProps) => {
  // Extract extension (case insensitive)
  const extension = filename.split(".").pop()?.toLowerCase() || "";

  // Mapping of extensions to Lucide icons
  const iconMap: Record<string, JSX.Element> = {
    // Images
    png: <Icon.ImageIcon {...rest} />,
    jpg: <Icon.ImageIcon {...rest} />,
    jpeg: <Icon.ImageIcon {...rest} />,
    gif: <Icon.ImageIcon {...rest} />,
    svg: <Icon.ImageIcon {...rest} />,
    webp: <Icon.ImageIcon {...rest} />,
    bmp: <Icon.ImageIcon {...rest} />,

    // Documents
    pdf: <Icon.BookTextIcon {...rest} />,
    doc: <Icon.FileTextIcon {...rest} />,
    docx: <Icon.FileTextIcon {...rest} />,
    txt: <Icon.FileTextIcon {...rest} />,
    rtf: <Icon.FileTextIcon {...rest} />,

    // Spreadsheets
    xls: <Icon.FileSpreadsheetIcon {...rest} />,
    xlsx: <Icon.FileSpreadsheetIcon {...rest} />,
    csv: <Icon.FileSpreadsheetIcon {...rest} />,

    // Archives
    zip: <Icon.FileArchiveIcon {...rest} />,
    rar: <Icon.FileArchiveIcon {...rest} />,
    "7z": <Icon.FileArchiveIcon {...rest} />,
    tar: <Icon.FileArchiveIcon {...rest} />,
    gz: <Icon.FileArchiveIcon {...rest} />,

    // Audio
    mp3: <Icon.FileAudioIcon {...rest} />,
    wav: <Icon.FileAudioIcon {...rest} />,
    ogg: <Icon.FileAudioIcon {...rest} />,
    flac: <Icon.FileAudioIcon {...rest} />,
    opus: <Icon.FileAudioIcon {...rest} />,

    // Video
    mp4: <Icon.FileVideoIcon {...rest} />,
    mov: <Icon.FileVideoIcon {...rest} />,
    avi: <Icon.FileVideoIcon {...rest} />,
    mkv: <Icon.FileVideoIcon {...rest} />,
    webm: <Icon.FileVideoIcon {...rest} />,

    // Code
    js: <Icon.FileCodeIcon {...rest} />,
    jsx: <Icon.FileCodeIcon {...rest} />,
    ts: <Icon.FileCodeIcon {...rest} />,
    tsx: <Icon.FileCodeIcon {...rest} />,
    html: <Icon.FileCodeIcon {...rest} />,
    css: <Icon.FileCodeIcon {...rest} />,
    scss: <Icon.FileCodeIcon {...rest} />,
    json: <Icon.FileJsonIcon {...rest} />,
    md: <Icon.FileType2Icon {...rest} />,
    py: <Icon.FileCodeIcon {...rest} />,
    rs: <Icon.FileCodeIcon {...rest} />,
    sh: <Icon.FileTerminalIcon {...rest} />,

    // Program
    exe: <Icon.AppWindow {...rest} />,
  };

  // Return matching icon or default file icon
  return iconMap[extension] || <Icon.FileIcon {...rest} />;
};

// inspired by https://github.com/AarambhDevHub/frontend-file-Chunks/blob/main/app/page.tsx
export const ResourceUploader = () => {
  const [progress, setProgress] = useState(0);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [resource, setResource] = useState<Resource | undefined>(undefined);

  const uploadFileInChunks = async () => {
    if (!selectedFile) {
      alert("Please select a file to upload");
      return;
    }

    const chunkSize = 1 * 1024 * 1024; // 1 MB
    const totalChunks = Math.ceil(selectedFile.size / chunkSize);
    let currentChunk = Number(localStorage.getItem(selectedFile.name)) || 0;
    const res_id = genId();

    setProgress(0);

    while (currentChunk < totalChunks) {
      const start = currentChunk * chunkSize;
      const end = Math.min(start + chunkSize, selectedFile.size);
      const chunk = selectedFile.slice(start, end);

      try {
        const { finished, resource } = await resourceUpload({
          chunk,
          filename: selectedFile.name,
          chunk_no: currentChunk,
          total_chunks: totalChunks,
          res_id,
          filetype: selectedFile.type,
          filesize: selectedFile.size,
          last_modified: selectedFile.lastModified,
        });

        if (resource) {
          setResource(resource);
        }

        currentChunk++;
        localStorage.setItem(selectedFile.name, currentChunk.toString());
        setProgress(parseInt(((currentChunk / totalChunks) * 100).toFixed(0)));
      } catch (error) {
        console.error(error);
        alert("An error occurred during upload.");
        return;
      }
    }

    localStorage.removeItem(selectedFile.name);
  };

  const handleFileChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      if (file) {
        setSelectedFile(file);
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
      setSelectedFile(file);
      setProgress(0);
    }
  }, []);

  return (
    <div className="max-w-3xl w-full align-center justify-center flex">
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

          <div className="flex flex-col items-center justify-center space-y-4">
            <div
              className={`
              p-3 rounded-2xl transition-transform duration-300
              ${isDragging ? "scale-110" : "scale-100"}
              ${
                selectedFile
                  ? "bg-green-100 dark:bg-green-900/30"
                  : "bg-blue-100 dark:bg-blue-900/30"
              }
            `}
            >
              <FileNameToIcon
                filename={selectedFile?.name ?? ""}
                className="w-50 h-50"
                strokeWidth={1}
              />
            </div>

            <div className="space-y-2">
              {selectedFile ? (
                <div className="text-gray-600 dark:text-gray-300 text-sm font-medium">
                  <div className="grid grid-cols-2 gap-2 text-gray-600 dark:text-gray-300 text-sm font-medium">
                    <div className="text-left font-semibold pr-4">Name:</div>
                    <div>{selectedFile.name}</div>

                    <div className="text-left font-semibold pr-4">Size:</div>
                    <div>{humanFileSize(selectedFile.size, false)}</div>

                    <div className="text-left font-semibold pr-4">Date:</div>
                    <div>
                      <RelativeTime
                        date={new Date(selectedFile.lastModified)}
                      />
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
              disabled={!selectedFile}
              className={
                "max-w-20 w-full py-3.5 px-6 rounded-lg align-center justify-center border kc-basic-with-bdr"
              }
            >
              {progress > 0 && progress < 100 ? <>Uploading...</> : "Upload"}
            </KButton>
          </div>
        )}
      </div>

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
