import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import type { KFileMeta } from "../../po";
import { inlineKFileDownload, inlineKFileUpload } from "../../service";
import { TextEditor } from "./editor";

type PlainTextProps = {
  kfile?: KFileMeta;
  readonly?: boolean;
  onUpload?: (content: string, contentType: string) => Promise<void>;
  onContentChange?: (content: string) => void;
};

export const PlainText = ({
  kfile,
  readonly,
  onUpload,
  onContentChange,
}: PlainTextProps) => {
  const [content, setContent] = useState<string>("");
  const [isLoading, setIsLoading] = useState(!!kfile);
  const saveTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const codeMirrorRef = useRef<ReactCodeMirrorRef>(null);

  useEffect(() => {
    if (kfile) {
      loadFileContent();
    }
  }, [kfile]);

  const loadFileContent = async () => {
    if (!kfile) return;
    try {
      setIsLoading(true);
      const { file } = await inlineKFileDownload({
        req_id: { Otid: kfile.otid },
      });
      if (file?.content) {
        setContent(file.content);
      }
    } catch (err) {
      console.error("Failed to load file:", err);
      toast.error("Failed to load file");
    } finally {
      setIsLoading(false);
    }
  };

  const handleSave = async (contentToSave: string) => {
    try {
      if (kfile) {
        await inlineKFileUpload({
          meta_id: kfile.id,
          otid: kfile.otid,
          res: {
            sid: kfile.sid,
            tid: kfile.tid,
            content: contentToSave,
          },
          archor_intervals: 0,
          content_type: "text/plain",
          binaryp: false,
        });
        toast.success("Saved successfully");
      } else if (onUpload) {
        await onUpload(contentToSave, "text/plain");
      }
    } catch (err) {
      console.error("Failed to save:", err);
      toast.error("Failed to save");
    }
  };

  const handleContentChange = (value: string) => {
    setContent(value);
    onContentChange?.(value);

    if (readonly) return;

    if (saveTimeoutRef.current) {
      clearTimeout(saveTimeoutRef.current);
    }

    saveTimeoutRef.current = setTimeout(() => {
      handleSave(value);
    }, 2000);
  };

  useEffect(() => {
    return () => {
      if (saveTimeoutRef.current) {
        clearTimeout(saveTimeoutRef.current);
      }
    };
  }, []);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-gray-500">Loading...</div>
      </div>
    );
  }

  if (readonly) {
    return (
      <div className="h-full w-full overflow-auto bg-white dark:bg-gray-900 p-4">
        <pre className="whitespace-pre-wrap font-mono text-sm">{content}</pre>
      </div>
    );
  }

  return (
    <TextEditor
      value={content}
      onChange={handleContentChange}
      placeholder="Enter your text..."
      ref={codeMirrorRef}
    />
  );
};
