import { exportToSvg } from "@excalidraw/excalidraw";
import type { LegacyAppState } from "@excalidraw/excalidraw/data/types";
import type { ExcalidrawElement } from "@excalidraw/excalidraw/element/types";
import type { AppState, BinaryFiles } from "@excalidraw/excalidraw/types";
import * as React from "react";
import { type JSX, useEffect, useState } from "react";
import type { ExcalidrawChnotState } from "../service";

// BEGIN lecixel
/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 */
type ImageType = "svg" | "canvas";

type Dimension = "inherit" | number;

type Props = {
  /**
   * Configures the export setting for SVG/Canvas
   */
  /**
   * The css class applied to image to be rendered
   */
  className?: string;
  elements?: readonly ExcalidrawElement[] | null;
  appState?: Readonly<
    Partial<
      AppState & {
        [T in keyof LegacyAppState]: LegacyAppState[T][0];
      }
    >
  > | null;
  files?: BinaryFiles;
  /**
   * The height of the image to be rendered
   */
  height?: Dimension;
  /**
   * The ref object to be used to render the image
   */
  imageContainerRef: React.RefObject<HTMLDivElement | null>;
  /**
   * The type of image to be rendered
   */
  imageType?: ImageType;
  /**
   * The css class applied to the root element of this component
   */
  rootClassName?: string | null;
  /**
   * The width of the image to be rendered
   */
  width?: Dimension;
};

// exportToSvg has fonts from excalidraw.com
// We don't want them to be used in open source
const removeStyleFromSvg_HACK = (svg: SVGElement) => {
  const styleTag = svg?.firstElementChild?.firstElementChild;

  // Generated SVG is getting double-sized by height and width attributes
  // We want to match the real size of the SVG element
  const viewBox = svg.getAttribute("viewBox");
  if (viewBox != null) {
    const viewBoxDimensions = viewBox.split(" ");
    const x = parseFloat(viewBoxDimensions[0]);
    const y = parseFloat(viewBoxDimensions[1]);
    const width = parseFloat(viewBoxDimensions[2]);
    const height = parseFloat(viewBoxDimensions[3]);

    // Define minimum viewBox dimensions
    const MIN_WIDTH = 1000;
    const MIN_HEIGHT = 50;

    let newX = x;
    let newY = y;
    let newWidth = width;
    let newHeight = height;

    // Add padding if viewBox is too small
    if (width < MIN_WIDTH || height < MIN_HEIGHT) {
      const paddingX = width < MIN_WIDTH ? (MIN_WIDTH - width) / 2 : 0;
      const paddingY = height < MIN_HEIGHT ? (MIN_HEIGHT - height) / 2 : 0;

      newX = x - paddingX;
      newY = y - paddingY;
      newWidth = width + paddingX * 2;
      newHeight = height + paddingY * 2;

      // Update viewBox with padding
      svg.setAttribute("viewBox", `${newX} ${newY} ${newWidth} ${newHeight}`);
    }

    svg.setAttribute("width", newWidth.toString());
    svg.setAttribute("height", newHeight.toString());
  }

  if (styleTag && styleTag.tagName === "style") {
    styleTag.remove();
  }
};

/**
 * @explorer-desc
 * A component for rendering Excalidraw elements as a static image
 */
export const ExcalidrawImage = ({
  elements,
  files,
  imageContainerRef,
  appState,
  rootClassName = null,
  width = "inherit",
  height = "inherit",
}: Props): JSX.Element => {
  const [svg, setSvg] = useState<SVGElement | null>(null);

  useEffect(() => {
    const setContent = async () => {
      const svg: SVGElement = await exportToSvg({
        appState,
        elements,
        files,
      });
      removeStyleFromSvg_HACK(svg);

      svg.setAttribute("width", "100%");
      svg.setAttribute("height", "100%");
      svg.setAttribute("display", "block");

      setSvg(svg);
    };
    setContent();
  }, [elements, files, appState]);

  const containerStyle: React.CSSProperties = {};
  if (width !== "inherit") {
    containerStyle.width = `${width}px`;
  }
  if (height !== "inherit") {
    containerStyle.height = `${height}px`;
  }

  return (
    <div
      ref={imageContainerRef}
      className={rootClassName ?? ""}
      style={containerStyle}
      // biome-ignore lint/security/noDangerouslySetInnerHtml: safe
      dangerouslySetInnerHTML={{ __html: svg?.outerHTML ?? "" }}
    />
  );
};

// END lecixel

const ExcalidrawPreview = ({
  state,
  className: rootClassName,
}: { state?: ExcalidrawChnotState } & Omit<Props, "imageContainerRef">) => {
  const imageContainerRef = React.useRef<HTMLDivElement>(null);

  return (
    <ExcalidrawImage
      appState={state?.appState}
      elements={state?.elements ?? []}
      files={state?.files}
      imageContainerRef={imageContainerRef}
      rootClassName={rootClassName}
    />
  );
};

export default ExcalidrawPreview;
