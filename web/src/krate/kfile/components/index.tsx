import { useCallback, useEffect, useState } from "react";
import Icon from "@/common/component/icon";
import { Button } from "@/common/component/ui/button";
import { useIsMobile } from "@/hooks/use-mobile";
import { chnotHeadStore } from "@/krate/chnot/store";
import { kfileMetaFetch, kfileUpload } from "@/krate/kfile/service";
import { genUID, type TID } from "@/lib/id_util";
import type { KFileMeta } from "../po";
import { CommonKFile } from "./common-kfile";
import { handleDownloadKfile } from "./download";
import { ImageKFile, isImageFile } from "./image-kfile";
import TextKFile, { isMermaidFile } from "./text-kfile";

type KFileViewerProps = {
  otid: TID;
  onPostSave?: (r: KFileMeta) => void;
  readonly?: boolean;
};

export const KFileViewer = ({
  otid,
  onPostSave,
  readonly,
}: KFileViewerProps) => {
  const [progress, setProgress] = useState(0);
  const [uploadFile, setUploadFile] = useState<File | undefined>(undefined);
  const [kfile, setKFile] = useState<KFileMeta | undefined>(undefined);
  const [isDragging, setIsDragging] = useState(false);
  const [isTextMode, setIsTextMode] = useState(false);
  const isMobile = useIsMobile();

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

    const chunkSize = 1 * 1024 * 1024;
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
          binaryp: true,
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
        setProgress(
          parseInt(((currentChunk / totalChunks) * 100).toFixed(0), 10),
        );
      } catch (_error) {
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

  const handleSelectFile = useCallback(() => {
    document.getElementById("file-input")?.click();
  }, []);

  const handleTextUpload = useCallback(
    async (content: string, contentType: string) => {
      const uploadId = genUID();
      const blob = new Blob([content], { type: contentType });

      const { kfile } = await kfileUpload({
        upload_id: uploadId,
        chunk: blob,
        filename: contentType === "text/mermaid" ? "diagram.mmd" : "text.txt",
        chunk_no: 0,
        total_chunks: 1,
        meta_id: uploadId,
        content_type: contentType,
        filesize: blob.size,
        last_modified: Date.now(),
        otid: otid,
        binaryp: false,
      });

      if (kfile) {
        if (onPostSave) {
          onPostSave(kfile);
        }
        setKFile(kfile);
      }
    },
    [otid, onPostSave],
  );

  useEffect(() => {
    const key = `kfile-${otid}`;
    const headerActions = (
      <>
        <input
          id="file-input-header"
          type="file"
          onChange={handleFileChange}
          className="hidden"
        />
        <Button
          variant="ghost"
          size="icon"
          onClick={() => document.getElementById("file-input-header")?.click()}
          title="Upload file"
        >
          <Icon.Upload />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setIsTextMode(true)}
          title="Text mode"
        >
          <Icon.FileText />
        </Button>
        {kfile && (
          <Button
            variant="ghost"
            size="icon"
            onClick={() => handleDownloadKfile(kfile)}
            title="Download"
          >
            <Icon.Download />
          </Button>
        )}
        {uploadFile && (
          <Button
            variant="ghost"
            size="icon"
            onClick={uploadFileInChunks}
            disabled={!uploadFile}
            title="Upload"
          >
            <Icon.Save />
          </Button>
        )}
      </>
    );

    chnotHeadStore.getState().registerHeaderActions(key, headerActions);

    return () => {
      chnotHeadStore.getState().unregisterHeaderActions(key);
    };
  }, [otid, kfile, uploadFile, handleFileChange, uploadFileInChunks]);

  if (isTextMode || kfile?.content_type.startsWith("text/")) {
    return (
      <TextKFile
        kfile={kfile}
        onUpload={handleTextUpload}
        onBack={() => setIsTextMode(false)}
        readonly={readonly}
      />
    );
  }

  if (kfile && isImageFile(kfile.content_type)) {
    return (
      <>
        <ImageKFile kfile={kfile} onReplace={handleSelectFile} />
      </>
    );
  }

  return (
    <CommonKFile
      kfile={kfile}
      uploadFile={uploadFile}
      progress={progress}
      isDragging={isDragging}
      isMobile={isMobile}
      onFileChange={handleFileChange}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      onUpload={uploadFileInChunks}
      onTextMode={() => setIsTextMode(true)}
      hideActionButtons
    />
  );
};
