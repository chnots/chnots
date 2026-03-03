import {
  AppWindow,
  BookTextIcon,
  FileArchiveIcon,
  FileAudioIcon,
  FileCodeIcon,
  FileIcon,
  FileJsonIcon,
  FileSpreadsheetIcon,
  FileTerminalIcon,
  FileTextIcon,
  FileType2Icon,
  FileVideoIcon,
  ImageIcon,
  type LucideProps,
} from "lucide-react";
import type { JSX } from "react";

const FileNameToIcon = ({
  filename,
  ...rest
}: {
  filename: string;
} & LucideProps) => {
  const extension = filename.split(".").pop()?.toLowerCase() || "";

  const iconMap: Record<string, JSX.Element> = {
    png: <ImageIcon {...rest} />,
    jpg: <ImageIcon {...rest} />,
    jpeg: <ImageIcon {...rest} />,
    gif: <ImageIcon {...rest} />,
    svg: <ImageIcon {...rest} />,
    webp: <ImageIcon {...rest} />,
    bmp: <ImageIcon {...rest} />,

    pdf: <BookTextIcon {...rest} />,
    doc: <FileTextIcon {...rest} />,
    docx: <FileTextIcon {...rest} />,
    txt: <FileTextIcon {...rest} />,
    rtf: <FileTextIcon {...rest} />,

    xls: <FileSpreadsheetIcon {...rest} />,
    xlsx: <FileSpreadsheetIcon {...rest} />,
    csv: <FileSpreadsheetIcon {...rest} />,

    zip: <FileArchiveIcon {...rest} />,
    rar: <FileArchiveIcon {...rest} />,
    "7z": <FileArchiveIcon {...rest} />,
    tar: <FileArchiveIcon {...rest} />,
    gz: <FileArchiveIcon {...rest} />,

    mp3: <FileAudioIcon {...rest} />,
    wav: <FileAudioIcon {...rest} />,
    ogg: <FileAudioIcon {...rest} />,
    flac: <FileAudioIcon {...rest} />,
    opus: <FileAudioIcon {...rest} />,

    mp4: <FileVideoIcon {...rest} />,
    mov: <FileVideoIcon {...rest} />,
    avi: <FileVideoIcon {...rest} />,
    mkv: <FileVideoIcon {...rest} />,
    webm: <FileVideoIcon {...rest} />,

    js: <FileCodeIcon {...rest} />,
    jsx: <FileCodeIcon {...rest} />,
    ts: <FileCodeIcon {...rest} />,
    tsx: <FileCodeIcon {...rest} />,
    html: <FileCodeIcon {...rest} />,
    css: <FileCodeIcon {...rest} />,
    scss: <FileCodeIcon {...rest} />,
    json: <FileJsonIcon {...rest} />,
    md: <FileType2Icon {...rest} />,
    py: <FileCodeIcon {...rest} />,
    rs: <FileCodeIcon {...rest} />,
    sh: <FileTerminalIcon {...rest} />,

    exe: <AppWindow {...rest} />,
  };

  return iconMap[extension] || <FileIcon {...rest} />;
};

export default FileNameToIcon;
