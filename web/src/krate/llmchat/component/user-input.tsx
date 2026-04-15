import { Image as ImageIcon, Paperclip, Send, X } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import { Button } from "@/common/component/ui/button";
import type { ContentBlock } from "@/krate/llmchat/po";
import LLMChatBotSelect from "./bot-select";

type AttachedFile = {
  id: string;
  file: File;
  preview?: string;
};

function fileToContentBlock(att: AttachedFile): ContentBlock {
  return {
    type: att.file.type.startsWith("image/") ? "image" : "file",
    data: att.preview ?? att.file.name,
    mediaType: att.file.type,
    filename: att.file.name,
  };
}

function readFileAsDataUrl(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
}

const UserInput = ({
  disabled,
  onAppendRecord,
}: {
  disabled: boolean;
  onAppendRecord: (content: string, attachments?: ContentBlock[]) => boolean;
}) => {
  const [message, setMessage] = useState<string>();
  const [attachments, setAttachments] = useState<AttachedFile[]>([]);
  const [dragOver, setDragOver] = useState(false);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const addFiles = useCallback(async (files: FileList | File[]) => {
    const newAtts: AttachedFile[] = [];
    for (const file of Array.from(files)) {
      let preview: string | undefined;
      if (file.type.startsWith("image/")) {
        preview = await readFileAsDataUrl(file);
      }
      newAtts.push({
        id: `${Date.now()}-${Math.random().toString(36).slice(2)}`,
        file,
        preview,
      });
    }
    setAttachments((prev) => [...prev, ...newAtts]);
  }, []);

  const removeAttachment = useCallback((id: string) => {
    setAttachments((prev) => prev.filter((a) => a.id !== id));
  }, []);

  const handleKeyDown = (e: {
    key: string;
    ctrlKey: boolean;
    preventDefault: () => void;
  }) => {
    if (e.key === "Enter" && e.ctrlKey && !disabled && message) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleSend = () => {
    const text = message ?? "";
    if (!text && attachments.length === 0) return;

    const extraBlocks =
      attachments.length > 0
        ? attachments.map((a) => fileToContentBlock(a))
        : undefined;

    if (onAppendRecord(text, extraBlocks)) {
      setMessage("");
      setAttachments([]);
    }
  };

  const handlePaste = useCallback(
    async (e: React.ClipboardEvent) => {
      const files: File[] = [];
      for (const item of Array.from(e.clipboardData.items)) {
        if (item.kind === "file") {
          const file = item.getAsFile();
          if (file) files.push(file);
        }
      }
      if (files.length > 0) {
        e.preventDefault();
        await addFiles(files);
      }
    },
    [addFiles],
  );

  const handleDrop = useCallback(
    async (e: React.DragEvent) => {
      e.preventDefault();
      setDragOver(false);
      if (e.dataTransfer.files.length > 0) {
        await addFiles(e.dataTransfer.files);
      }
    },
    [addFiles],
  );

  useEffect(() => {
    if (textareaRef.current) {
      if (message?.length === 0) {
        textareaRef.current.style.height = "auto";
      } else {
        textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
      }
    }
  }, [message]);

  return (
    <section
      aria-label="Message input with file drop zone"
      className={`pl-3 p-1 flex justify-center space-x-2 mb-2 ${dragOver ? "ring-2 ring-primary rounded-lg" : ""}`}
      onDragOver={(e) => {
        e.preventDefault();
        setDragOver(true);
      }}
      onDragLeave={() => setDragOver(false)}
      onDrop={handleDrop}
    >
      <div className="flex flex-col max-w-3xl w-3xl p-2 rounded-xl border shadow-xl">
        {attachments.length > 0 && (
          <div className="flex flex-wrap gap-2 p-1">
            {attachments.map((att) => (
              <div
                key={att.id}
                className="relative group flex items-center gap-1 rounded-md border bg-muted/50 px-2 py-1 text-xs"
              >
                {att.preview ? (
                  <img
                    src={att.preview}
                    alt={att.file.name}
                    className="h-8 w-8 rounded object-cover"
                  />
                ) : (
                  <Paperclip className="h-3 w-3" />
                )}
                <span className="max-w-24 truncate">{att.file.name}</span>
                <button
                  type="button"
                  onClick={() => removeAttachment(att.id)}
                  className="ml-1 text-muted-foreground hover:text-foreground"
                >
                  <X className="h-3 w-3" />
                </button>
              </div>
            ))}
          </div>
        )}
        <textarea
          className="w-full p-1 h-auto max-h-60 border-none focus:outline-none focus:none resize-none"
          onChange={(e) => {
            setMessage(e.target.value);
          }}
          value={message}
          onKeyDown={handleKeyDown}
          onPaste={handlePaste}
          placeholder="Type your question... (drag & drop or paste files)"
          ref={textareaRef}
        />
        <div className="flex justify-between">
          <div className="flex gap-1">
            <LLMChatBotSelect />
            <Button
              variant="ghost"
              size="icon"
              onClick={() => fileInputRef.current?.click()}
              disabled={disabled}
            >
              <ImageIcon className="w-4 h-4" />
            </Button>
            <input
              ref={fileInputRef}
              type="file"
              multiple
              className="hidden"
              onChange={(e) => {
                if (e.target.files) addFiles(e.target.files);
                e.target.value = "";
              }}
            />
          </div>
          <Button
            onClick={handleSend}
            disabled={disabled && attachments.length === 0}
          >
            <Send className="w-4 h-4" />
          </Button>
        </div>
      </div>
    </section>
  );
};

export default UserInput;
