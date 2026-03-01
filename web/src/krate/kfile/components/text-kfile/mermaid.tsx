import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import mermaid from "mermaid";
import { useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import type { KFileMeta } from "../../po";
import { inlineKFileDownload, inlineKFileUpload } from "../../service";
import { TextEditor } from "./editor";
import { SvgPreview } from "./svg-preview";

mermaid.initialize({
  startOnLoad: false,
  theme: "default",
  securityLevel: "loose",
});

type MermaidTextProps = {
  kfile?: KFileMeta;
  readonly?: boolean;
  onUpload?: (content: string, contentType: string) => Promise<void>;
  onContentChange?: (content: string) => void;
};

export const MermaidText = ({
  kfile,
  readonly: initialReadonly,
  onUpload,
  onContentChange,
}: MermaidTextProps) => {
  const [content, setContent] = useState<string>("");
  const [svgContent, setSvgContent] = useState<string>("");
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(!!kfile);
  const saveTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const codeMirrorRef = useRef<ReactCodeMirrorRef>(null);
  const [readonly, setReadonly] = useState(initialReadonly);

  useEffect(() => {
    if (kfile) {
      loadFileContent();
    }
  }, [kfile]);

  useEffect(() => {
    if (content) {
      renderMermaid();
    }
  }, [content]);

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
      console.error("Failed to load mermaid file:", err);
      toast.error("Failed to load mermaid file");
    } finally {
      setIsLoading(false);
    }
  };

  const renderMermaid = async () => {
    try {
      setError(null);
      const { svg } = await mermaid.render(
        `mermaid-preview-${Date.now()}`,
        content,
      );
      setSvgContent(svg);
    } catch (err) {
      console.error("Mermaid render error:", err);
      setError(err instanceof Error ? err.message : "Failed to render diagram");
      setSvgContent("");
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
          content_type: "text/mermaid",
          binaryp: false,
        });
        toast.success("Saved successfully");
      } else if (onUpload) {
        await onUpload(contentToSave, "text/mermaid");
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
      <div className="h-full w-full">
        {error ? (
          <div className="h-full overflow-auto p-4">
            <div className="text-red-500 p-4 border border-red-300 rounded bg-red-50 dark:bg-red-900/20 dark:border-red-800">
              <h3 className="font-bold mb-2">Error:</h3>
              <pre className="text-sm overflow-auto">{error}</pre>
            </div>
          </div>
        ) : (
          <SvgPreview
            svgContent={svgContent}
            className="h-full"
            onHideEditor={() => setReadonly((prev) => !prev)}
          />
        )}
      </div>
    );
  }

  return (
    <div className="flex h-full">
      <div className="w-1/2 border-r border-gray-200 dark:border-gray-700 flex flex-col">
        <TextEditor
          value={content}
          onChange={handleContentChange}
          placeholder="Enter your Mermaid diagram code..."
          ref={codeMirrorRef}
        />
      </div>

      <div className="w-1/2">
        {error ? (
          <div className="h-full overflow-auto p-4">
            <div className="text-red-500 p-4 border border-red-300 rounded bg-red-50 dark:bg-red-900/20 dark:border-red-800">
              <h3 className="font-bold mb-2">Error:</h3>
              <pre className="text-sm overflow-auto">{error}</pre>
            </div>
          </div>
        ) : (
          <SvgPreview
            svgContent={svgContent}
            className="h-full"
            onHideEditor={() => setReadonly((prev) => !prev)}
          />
        )}
      </div>
    </div>
  );
};

export const isMermaidFile = (contentType?: string): boolean => {
  return contentType === "text/mermaid";
};
