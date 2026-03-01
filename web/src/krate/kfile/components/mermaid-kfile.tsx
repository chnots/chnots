import { EditorView } from "@codemirror/view";
import CodeMirror from "@uiw/react-codemirror";
import { Save } from "lucide-react";
import mermaid from "mermaid";
import { useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import { Button } from "@/common/component/ui/button";
import { cn } from "@/lib/utils";
import type { KFileMeta } from "../po";
import { inlineKFileDownload, inlineKFileUpload } from "../service";

mermaid.initialize({
  startOnLoad: false,
  theme: "default",
  securityLevel: "loose",
});

type MermaidKFileProps = {
  kfile: KFileMeta;
  onReplace?: () => void;
};

const MermaidKFile = ({ kfile }: MermaidKFileProps) => {
  const [content, setContent] = useState<string>("");
  const [svgContent, setSvgContent] = useState<string>("");
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const previewRef = useRef<HTMLDivElement>(null);
  const saveTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    loadFileContent();
  }, [kfile]);

  useEffect(() => {
    if (content) {
      renderMermaid();
    }
  }, [content]);

  const loadFileContent = async () => {
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
      const { svg } = await mermaid.render("mermaid-preview", content);
      setSvgContent(svg);
    } catch (err) {
      console.error("Mermaid render error:", err);
      setError(err instanceof Error ? err.message : "Failed to render diagram");
      setSvgContent("");
    }
  };

  const handleSave = async () => {
    try {
      setIsSaving(true);
      await inlineKFileUpload({
        meta_id: kfile.id,
        otid: kfile.otid,
        res: {
          sid: kfile.sid,
          tid: kfile.tid,
          content: content,
        },
        archor_intervals: 0,
        content_type: "text/mermaid",
        binaryp: false,
      });
      toast.success("Saved successfully");
    } catch (err) {
      console.error("Failed to save:", err);
      toast.error("Failed to save");
    } finally {
      setIsSaving(false);
    }
  };

  const handleContentChange = (value: string) => {
    setContent(value);

    if (saveTimeoutRef.current) {
      clearTimeout(saveTimeoutRef.current);
    }

    saveTimeoutRef.current = setTimeout(() => {
      handleSave();
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

  return (
    <div className="flex flex-col h-full">
      <div className="flex justify-end p-2 border-b border-gray-200 dark:border-gray-700">
        <Button
          onClick={handleSave}
          disabled={isSaving}
          size="sm"
          variant="outline"
        >
          <Save className="w-4 h-4 mr-2" />
          {isSaving ? "Saving..." : "Save"}
        </Button>
      </div>

      <div className="flex flex-1 overflow-hidden">
        <div className="w-1/2 border-r border-gray-200 dark:border-gray-700 overflow-auto">
          <CodeMirror
            value={content}
            height="100%"
            extensions={[EditorView.lineWrapping]}
            onChange={handleContentChange}
            basicSetup={{
              lineNumbers: true,
              highlightActiveLineGutter: true,
              foldGutter: true,
              closeBrackets: true,
            }}
            placeholder="Enter your Mermaid diagram code..."
            className={cn(
              "h-full",
              "[&_.cm-editor]:h-full",
              "[&_.cm-scroller]:h-full",
            )}
          />
        </div>

        <div className="w-1/2 overflow-auto p-4 bg-white dark:bg-gray-900">
          {error ? (
            <div className="text-red-500 p-4 border border-red-300 rounded bg-red-50 dark:bg-red-900/20 dark:border-red-800">
              <h3 className="font-bold mb-2">Error:</h3>
              <pre className="text-sm overflow-auto">{error}</pre>
            </div>
          ) : (
            <div
              ref={previewRef}
              className="mermaid-preview flex items-center justify-center min-h-full"
              dangerouslySetInnerHTML={{ __html: svgContent }}
            />
          )}
        </div>
      </div>
    </div>
  );
};

export const isMermaidFile = (contentType?: string): boolean => {
  return contentType === "text/mermaid";
};

export default MermaidKFile;
