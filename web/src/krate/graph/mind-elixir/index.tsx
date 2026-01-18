import React, {
  useEffect,
  useRef,
  useCallback,
  useMemo,
  type CSSProperties,
} from "react";
import MindElixir, {
  type MindElixirData,
  type MindElixirInstance,
  type Options,
} from "mind-elixir";
import "mind-elixir/style.css";

type MindElixirPlugin = (instance: MindElixirInstance) => void;

type OperationType = "expand" | "changeDirection" | "operation";

function generateUUID() {
  const array = new Uint8Array(16);
  crypto.getRandomValues(array);

  // Set version (4) and variant (RFC 4122 compliant)
  array[6] = (array[6] & 0x0f) | 0x40; // Version 4
  array[8] = (array[8] & 0x3f) | 0x80; // Variant

  return [...array]
    .map((byte, index) => byte.toString(16).padStart(2, "0"))
    .join("");
}

export type MindElixirReactProps = {
  style?: CSSProperties;
  data?: MindElixirData;
  plugins?: MindElixirPlugin[];
  onOperate?: (operation: unknown) => void;
  onSelectNode?: (operation: unknown) => void;
  onExpandNode?: (operation: unknown) => void;
  onChangeDirection?: (direction: number) => void;
  onChanged?: (data: MindElixirData, operationType: OperationType) => void;
  onHandleImage?: (file: File) => Promise<{ url: string }>;
  onPaste?: (e: ClipboardEvent) => void;
} & Partial<Options>;

export interface MindElixirReactRef {
  instance: MindElixirInstance | null;
  container: HTMLDivElement;
}

const MindElixirReact = React.forwardRef(
  (
    {
      style,
      data: initialData,
      plugins = [],
      onOperate,
      onSelectNode,
      onExpandNode,
      onChangeDirection,
      onChanged,
      onHandleImage,
      imageProxy,
      onPaste,
      ...restProps
    }: MindElixirReactProps,
    ref,
  ) => {
    const mindElixirInstance = useRef<MindElixirInstance>(null);
    const data = initialData ?? {
      direction: 1,
      nodeData: {
        id: generateUUID(),
        topic: "Root",
      },
    };

    const containerRef = useRef<HTMLDivElement>(null);
    const isFirstRun = useRef(true);

    const containerStyle = useMemo(
      () => ({
        width: "100%",
        height: "100%",
        ...style,
      }),
      [style],
    );

    const initializeMindElixir = useCallback(() => {
      if (!containerRef.current) return null;
      console.log("onHandleImage", onHandleImage);

      try {
        const instance = new MindElixir({
          ...restProps,
          el: containerRef.current,
          direction: 2 as const, // 0 left, 1 right, 2 both sides
          draggable: true,
          toolBar: true,
          keypress: true,
          locale: "en" as const,
          editable: true,
          allowUndo: true,
          contextMenu: true,
          imageProxy,
          pasteHandler: onPaste,
        });

        plugins.forEach((plugin) => {
          if (plugin) {
            instance.install(plugin);
          }
        });

        return instance;
      } catch (error) {
        console.error("Failed to initialize MindElixir:", error);
        return null;
      }
    }, [onHandleImage, imageProxy, plugins]);

    const setupEventListeners = useCallback(
      (instance: MindElixirInstance) => {
        if (!instance || !instance.bus) return;

        const handleOperation = (operation: unknown) => {
          if (onChanged) onChanged(instance.getData(), "operation");
          if (onOperate) onOperate(operation);
        };

        const handleSelectNode = (operation: unknown) => {
          if (onSelectNode) onSelectNode(operation);
        };

        const handleExpandNode = (operation: unknown) => {
          if (onChanged) onChanged(instance.getData(), "expand");
          if (onExpandNode) onExpandNode(operation);
        };

        const handleChangeDirection = (direction: number) => {
          if (onChanged) onChanged(instance.getData(), "changeDirection");
          if (onChangeDirection) onChangeDirection(direction);
        };

        instance.bus.addListener("operation", handleOperation);
        instance.bus.addListener("selectNodes", handleSelectNode);
        instance.bus.addListener("expandNode", handleExpandNode);
        instance.bus.addListener("changeDirection", handleChangeDirection);

        return () => {
          instance.bus.removeListener("operation", handleOperation);
          instance.bus.removeListener("selectNodes", handleSelectNode);
          instance.bus.removeListener("expandNode", handleExpandNode);
          instance.bus.removeListener("changeDirection", handleChangeDirection);
        };
      },
      [onOperate, onSelectNode, onExpandNode, onChangeDirection],
    );

    const handleDataUpdate = useCallback(
      (
        instance: MindElixirInstance,
        newData: MindElixirData,
        isInitial = false,
      ) => {
        if (!instance) return;

        try {
          if (isInitial) {
            instance.init(newData);
          } else {
            instance.refresh(newData);
          }
        } catch (error) {
          console.error("Failed to update MindElixir data:", error);
        }
      },
      [],
    );

    useEffect(() => {
      if (!containerRef.current) return;

      const instance = initializeMindElixir();
      if (!instance) return;

      mindElixirInstance.current = instance;

      const cleanupListeners = setupEventListeners(instance);

      if (ref) {
        if (typeof ref === "function") {
          ref({
            instance,
            container: containerRef.current,
          });
        } else if (ref) {
          ref.current = {
            instance,
            container: containerRef.current,
          };
        }
      }

      return () => {
        if (cleanupListeners) cleanupListeners();

        if (instance.destroy && typeof instance.destroy === "function") {
          instance.destroy();
        }
        mindElixirInstance.current = null;
      };
    }, [initializeMindElixir, setupEventListeners, ref]);

    useEffect(() => {
      if (!mindElixirInstance.current) return;

      const initializeWithData = async () => {
        const instance = mindElixirInstance.current;
        if (!instance) return;

        if (data) {
          handleDataUpdate(instance, data, true);
          isFirstRun.current = false;
        }
      };

      initializeWithData();
    }, [data, handleDataUpdate]);

    const setRefs = useCallback((node: HTMLDivElement) => {
      containerRef.current = node;
    }, []);

    return (
      <div
        ref={setRefs}
        style={containerStyle}
        className="mind-elixir-container"
      />
    );
  },
);

MindElixirReact.displayName = "MindElixirReact";

export type { MindElixirData };
export type { MindElixirPlugin };
export default MindElixirReact;
