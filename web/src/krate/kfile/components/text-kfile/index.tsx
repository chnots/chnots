import { FileText, Save } from "lucide-react";
import { useState } from "react";
import { Button } from "@/common/component/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/common/component/ui/select";
import type { KFileMeta } from "../../po";
import { isMermaidFile, MermaidText } from "./mermaid";
import { PlainText } from "./plain";

type TextType = "plain" | "mermaid";

type TextKFileProps = {
  kfile?: KFileMeta;
  readonly?: boolean;
  onUpload?: (content: string, contentType: string) => Promise<void>;
  onBack?: () => void;
};

const TextKFile = ({ kfile, readonly, onUpload, onBack }: TextKFileProps) => {
  const [textType, setTextType] = useState<TextType>(() => {
    if (kfile && isMermaidFile(kfile.content_type)) {
      return "mermaid";
    }
    return "plain";
  });
  const [isSaving, setIsSaving] = useState(false);
  const [currentContent, setCurrentContent] = useState<string>("");

  const showTypeSelector = !kfile;

  const handleManualSave = async () => {
    setIsSaving(true);
    setTimeout(() => setIsSaving(false), 100);
  };

  const handleContentChange = (content: string) => {
    setCurrentContent(content);
  };

  return (
    <div className="flex flex-col h-full w-full">
      <div className="flex justify-between items-center p-2 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center gap-2">
          {onBack && (
            <Button onClick={onBack} size="sm" variant="ghost">
              <FileText className="w-4 h-4 mr-2" />
              File Upload
            </Button>
          )}
          {showTypeSelector && (
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
          )}
        </div>

        {!readonly && (
          <Button
            onClick={handleManualSave}
            disabled={isSaving || !currentContent}
            size="sm"
            variant="outline"
          >
            <Save className="w-4 h-4 mr-2" />
            {isSaving ? "Saving..." : "Save"}
          </Button>
        )}
      </div>

      <div className="flex-1 overflow-hidden">
        {textType === "mermaid" ? (
          <MermaidText
            kfile={kfile}
            readonly={readonly}
            onUpload={onUpload}
            onContentChange={handleContentChange}
          />
        ) : (
          <PlainText
            kfile={kfile}
            readonly={readonly}
            onUpload={onUpload}
            onContentChange={handleContentChange}
          />
        )}
      </div>
    </div>
  );
};

export default TextKFile;

export { isMermaidFile };
