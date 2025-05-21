import Icon from "@/common/component/icon";
import { JSX } from "react";

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

export default FileNameToIcon;
