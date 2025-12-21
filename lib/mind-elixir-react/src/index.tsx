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

export interface MindElixirReactProps {
  style?: CSSProperties;
  data?: MindElixirData;
  options?: Omit<Options, "el">;
  plugins?: MindElixirPlugin[];
  onOperate?: (operation: unknown) => void;
  onSelectNode?: (operation: unknown) => void;
  onExpandNode?: (operation: unknown) => void;
  onChanged?: (data: MindElixirData) => void;
}

export interface MindElixirReactRef {
  instance: MindElixirInstance | null;
  container: HTMLDivElement | null;
  save: () => Promise<boolean>;
}

const MindElixirReact = React.forwardRef(
  (
    {
      style,
      data,
      options = {},
      plugins = [],
      onOperate,
      onSelectNode,
      onExpandNode,
      onChanged,
      ...restProps
    }: MindElixirReactProps,
    ref,
  ) => {
    console.log("beginto load mind elixir");
    // Default options for better UX
    const defaultOptions = useMemo(
      () => ({
        direction: 2 as const, // 0 left, 1 right, 2 both sides
        draggable: true,
        toolBar: true,
        nodeMenu: true,
        keypress: true,
        locale: "en" as const,
        editable: true,
        contextMenu: true,
        ...options,
      }),
      [options],
    );

    const containerRef = useRef<HTMLElement>(null);
    const mindElixirInstance = useRef<MindElixirInstance>(null);
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

      try {
        const instance = new MindElixir({
          el: containerRef.current,
          ...defaultOptions,
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
    }, [defaultOptions, plugins]);

    const setupEventListeners = useCallback(
      (instance: MindElixirInstance) => {
        if (!instance || !instance.bus) return;

        const handleOperation = (operation: unknown) => {
          console.log("handle operation", mindElixirInstance.current?.getData())
          if (onOperate) onOperate(operation);
        };

        const handleSelectNode = (operation: unknown) => {
          if (onSelectNode) onSelectNode(operation);
        };

        const handleExpandNode = (operation: unknown) => {
          if (onExpandNode) onExpandNode(operation);
        };

        instance.bus.addListener("operation", handleOperation);
        // @ts-expect-error
        instance.bus.addListener("selectNode", handleSelectNode);
        instance.bus.addListener("expandNode", handleExpandNode);

        return () => {
          instance.bus.removeListener("operation", handleOperation);
          // @ts-expect-error
          instance.bus.removeListener("selectNode", handleSelectNode);
          instance.bus.removeListener("expandNode", handleExpandNode);
        };
      },
      [onOperate, onSelectNode, onExpandNode],
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
            console.log("MindElixir initialized with data");
          } else {
            instance.refresh(newData);
            console.log("MindElixir data refreshed");
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
            save: () => onChanged && onChanged(instance.getData()),
          });
        } else if (ref) {
          ref.current = {
            instance,
            container: containerRef.current,
            save: () => onChanged && onChanged(instance.getData()),
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
    }, [
      initializeMindElixir,
      setupEventListeners,
      ref
    ]);

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

    const setRefs = useCallback(
      (node: HTMLDivElement) => {
        console.log("set re")
        containerRef.current = node;

        if (ref) {
          if (typeof ref === "function") {
            ref(node);
          } else if (ref) {
            ref.current = node as unknown as HTMLDivElement;
          }
        }
      },
      [ref],
    );

    return (
      <div
        ref={setRefs}
        style={containerStyle}
        className="mind-elixir-container"
        {...restProps}
      />
    );
  },
);

MindElixirReact.displayName = "MindElixirReact";

export type { MindElixirData };
export type { MindElixirPlugin };
export default MindElixirReact;
