import { EditorView } from "@codemirror/view";
import CodeMirror from "@uiw/react-codemirror";
import { FileText, Save } from "lucide-react";
import mermaid from "mermaid";
import { useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import { Button } from "@/common/component/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/common/component/ui/select";
import { cn } from "@/lib/utils";

mermaid.initialize({
  startOnLoad: false,
  theme: "default",
  securityLevel: "loose",
});

type TextType = "plain" | "mermaid";

type TextKFileProps = {
  onUpload: (content: string, contentType: string) => Promise<void>;
  onBack?: () => void;
};

const TextKFile = ({ onUpload, onBack }: TextKFileProps) => {
  const [content, setContent] = useState<string>("");
  const [textType, setTextType] = useState<TextType>("plain");
  const [svgContent, setSvgContent] = useState<string>("");
  const [error, setError] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);
  const previewRef = useRef<HTMLDivElement>(null);
  const saveTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    if (textType === "mermaid" && content) {
      renderMermaid();
    }
  }, [content, textType]);

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

  const handleSave = async () => {
    try {
      setIsSaving(true);
      const contentType =
        textType === "mermaid" ? "text/mermaid" : "text/plain";
      await onUpload(content, contentType);
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

  const showPreview = textType === "mermaid";

  return (
    <div className="flex flex-col h-full">
      <div className="flex justify-between items-center p-2 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center gap-2">
          {onBack && (
            <Button onClick={onBack} size="sm" variant="ghost">
              <FileText className="w-4 h-4 mr-2" />
              File Upload
            </Button>
          )}
          <Select
            value={textType}
            onValueChange={(value) => setTextType(value as TextType)}
          >
            <SelectTrigger size="sm" className="w-[140px]">
              <SelectValue placeholder="Select type" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="plain">Plain Text</SelectItem>
              <SelectItem value="mermaid">Mermaid</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <Button
          onClick={handleSave}
          disabled={isSaving || !content}
          size="sm"
          variant="outline"
        >
          <Save className="w-4 h-4 mr-2" />
          {isSaving ? "Saving..." : "Save"}
        </Button>
      </div>

      <div className="flex flex-1 overflow-hidden">
        <div
          className={cn(
            "overflow-auto",
            showPreview
              ? "w-1/2 border-r border-gray-200 dark:border-gray-700"
              : "w-full",
          )}
        >
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
            placeholder={
              textType === "mermaid"
                ? "Enter your Mermaid diagram code..."
                : "Enter your text..."
            }
            className={cn(
              "h-full",
              "[&_.cm-editor]:h-full",
              "[&_.cm-scroller]:h-full",
            )}
          />
        </div>

        {showPreview && (
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
        )}
      </div>
    </div>
  );
};

export default TextKFile;
