import { snapdom } from "@zumer/snapdom";
import MindElixir, {
  type MindElixirData,
  type MindElixirInstance,
  type Options,
} from "mind-elixir";
import "mind-elixir/style.css";
import { useEffect, useRef, useState } from "react";

const MindElixirPreview = ({ data }: { data: MindElixirData }) => {
  const [htmlImageElement, setPreview] = useState<HTMLImageElement>();

  useEffect(() => {
    (async () => {
      const container = document.createElement("div");
      container.id = "mindmao-elixir-temp";
      container.style.position = "absolute";
      container.style.left = "-9999px";
      container.style.top = "-9999px";
      container.style.width = "1000px";
      container.style.height = "1000px";
      document.body.appendChild(container);

      try {
        const instance = new MindElixir({
          el: container,
          direction: 2 as const,
          draggable: false,
          toolBar: false,
          keypress: false,
          locale: "en" as const,
          editable: false,
          allowUndo: false,
          contextMenu: false,
        });
        instance.init(data);

        await new Promise((resolve) => setTimeout(resolve, 0));

        const result = await snapdom(instance.nodes);
        const imgElement = await result.toSvg({ width: 100, height: 100 });
        setPreview(imgElement);
      } finally {
        document.body.removeChild(container);
      }
    })();
  }, [data]);

  return htmlImageElement ? (
    <img
      src={htmlImageElement.src}
      alt={htmlImageElement.alt}
      width={htmlImageElement.width || undefined}
      height={htmlImageElement.height || undefined}
      className={htmlImageElement.className}
    />
  ) : (
    <div>Loading</div>
  );
};

export default MindElixirPreview;
