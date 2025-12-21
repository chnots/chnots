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

type MindElixirPlugin = (instance: MindElixirInstance) => void;

export interface MindElixirReactProps {
  style?: CSSProperties;

  data?: MindElixirData;

  options?: Omit<Options, "el">;

  plugins?: MindElixirPlugin[];

  onOperate?: (operation: any) => void;

  onSelectNode?: (operation: any) => void;

  onExpandNode?: (operation: any) => void;
}

// 暴露给父组件的 ref 类型
export interface MindElixirReactRef {
  /** MindElixir 实例 */
  instance: MindElixirInstance | null;
  /** 容器 DOM 元素 */
  container: HTMLDivElement | null;
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
      ...restProps
    }: MindElixirReactProps,
    ref,
  ) => {
    const containerRef = useRef<HTMLElement>(null);
    const mindElixirInstance = useRef<MindElixirInstance>(null);
    const isFirstRun = useRef(true);

    // 合并默认样式
    const containerStyle = useMemo(
      () => ({
        width: "100%",
        height: "100%",
        ...style,
      }),
      [style],
    );

    // 初始化 MindElixir 实例
    const initializeMindElixir = useCallback(() => {
      if (!containerRef.current) return null;

      try {
        const instance = new MindElixir({
          el: containerRef.current,
          ...options,
        });

        // 安装插件
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
    }, [options, plugins]);

    // 设置事件监听器
    const setupEventListeners = useCallback(
      (instance: MindElixirInstance) => {
        if (!instance || !instance.bus) return;

        const handleOperation = (operation: any) => {
          if (onOperate) onOperate(operation);
        };

        const handleSelectNode = (operation: any) => {
          if (onSelectNode) onSelectNode(operation);
        };

        const handleExpandNode = (operation: any) => {
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

    // 初始化实例和事件监听器
    useEffect(() => {
      if (!containerRef.current) return;

      const instance = initializeMindElixir();
      if (!instance) return;

      mindElixirInstance.current = instance;

      // 设置事件监听器并获取清理函数
      const cleanupListeners = setupEventListeners(instance);

      // 暴露实例给父组件
      if (ref) {
        if (typeof ref === "function") {
          ref({ instance, container: containerRef.current });
        } else if (ref) {
          ref.current = { instance, container: containerRef.current };
        }
      }

      // 清理函数
      return () => {
        if (cleanupListeners) cleanupListeners();

        // 清理实例
        if (instance.destroy && typeof instance.destroy === "function") {
          instance.destroy();
        }
        mindElixirInstance.current = null;
      };
    }, [initializeMindElixir, setupEventListeners, ref]);

    // 处理数据变化
    useEffect(() => {
      if (!mindElixirInstance.current || !data) return;

      handleDataUpdate(mindElixirInstance.current, data, isFirstRun.current);

      if (isFirstRun.current) {
        isFirstRun.current = false;
      }
    }, [data, handleDataUpdate]);

    // 处理 ref 转发
    const setRefs = useCallback(
      (node: HTMLDivElement) => {
        containerRef.current = node;

        // 处理 forwardRef
        if (ref) {
          if (typeof ref === "function") {
            ref(node);
          } else if (ref) {
            ref.current = node;
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

export default MindElixirReact;
