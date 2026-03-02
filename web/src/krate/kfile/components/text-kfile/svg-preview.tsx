import Panzoom from "@panzoom/panzoom";
import {
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  ChevronLeft,
  ChevronRight,
  Download,
  Hand,
  Maximize2,
  Minus,
  PenOff,
  Plus,
  RotateCcw,
  Search,
  X,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { Button } from "@/common/component/ui/button";
import { Input } from "@/common/component/ui/input";
import { cn } from "@/lib/utils";

type PanzoomType = ReturnType<typeof Panzoom>;

type SvgPreviewProps = {
  svgContent: string;
  className?: string;
  onHideEditor: () => void;
};

type MatchInfo = {
  element: Element;
  originalFill: string | null;
  originalFontWeight: string | null;
  index: number;
};

export const SvgPreview = ({
  svgContent,
  className,
  onHideEditor,
}: SvgPreviewProps) => {
  const [searchText, setSearchText] = useState("");
  const [showSearch, setShowSearch] = useState(false);
  const [matches, setMatches] = useState<MatchInfo[]>([]);
  const [currentMatchIndex, setCurrentMatchIndex] = useState(-1);
  const [mouseInteractionEnabled, setMouseInteractionEnabled] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const panzoomRef = useRef<PanzoomType | null>(null);
  const matchesRef = useRef<MatchInfo[]>([]);

  useEffect(() => {
    matchesRef.current = matches;
  }, [matches]);

  useEffect(() => {
    if (!containerRef.current) return;

    const panzoom = Panzoom(containerRef.current, {
      contain: "outside",
      maxScale: 10,
      minScale: 0.1,
      startScale: 1,
      disablePan: !mouseInteractionEnabled,
      disableZoom: !mouseInteractionEnabled,
      cursor: mouseInteractionEnabled ? "grab" : "default",
    });

    panzoomRef.current = panzoom;

    const parent = containerRef.current.parentElement;
    if (parent && mouseInteractionEnabled) {
      parent.addEventListener("wheel", panzoom.zoomWithWheel);
    }

    return () => {
      if (parent && mouseInteractionEnabled) {
        parent.removeEventListener("wheel", panzoom.zoomWithWheel);
      }
      panzoom.destroy();
    };
  }, [mouseInteractionEnabled]);

  const clearAllHighlights = () => {
    matchesRef.current.forEach((match) => {
      if (match.originalFill) {
        match.element.setAttribute("fill", match.originalFill);
      } else {
        match.element.removeAttribute("fill");
      }
      if (match.originalFontWeight) {
        match.element.setAttribute("font-weight", match.originalFontWeight);
      } else {
        match.element.removeAttribute("font-weight");
      }
    });
  };

  const highlightCurrentMatch = (index: number) => {
    clearAllHighlights();

    matchesRef.current.forEach((match, i) => {
      if (i === index) {
        match.element.setAttribute("fill", "#ff6b6b");
        match.element.setAttribute("font-weight", "bold");
        match.element.scrollIntoView({ behavior: "smooth", block: "center" });
      } else {
        match.element.setAttribute("fill", "#ffa500");
        match.element.setAttribute("font-weight", "bold");
      }
    });
  };

  const handleSearch = () => {
    if (!searchText || !containerRef.current) {
      clearMatches();
      return;
    }

    const svgElement = containerRef.current.querySelector("svg");
    if (!svgElement) return;

    clearAllHighlights();

    const textElements = svgElement.querySelectorAll("text, tspan");
    const newMatches: MatchInfo[] = [];
    let matchIndex = 0;

    textElements.forEach((el) => {
      const text = el.textContent || "";
      if (text.toLowerCase().includes(searchText.toLowerCase())) {
        newMatches.push({
          element: el,
          originalFill: el.getAttribute("fill"),
          originalFontWeight: el.getAttribute("font-weight"),
          index: matchIndex++,
        });
      }
    });

    setMatches(newMatches);
    matchesRef.current = newMatches;

    if (newMatches.length > 0) {
      setCurrentMatchIndex(0);
      highlightCurrentMatch(0);
    } else {
      setCurrentMatchIndex(-1);
    }
  };

  const clearMatches = () => {
    clearAllHighlights();
    setMatches([]);
    setCurrentMatchIndex(-1);
    matchesRef.current = [];
  };

  const handleNextMatch = () => {
    if (matches.length === 0) return;
    const nextIndex = (currentMatchIndex + 1) % matches.length;
    setCurrentMatchIndex(nextIndex);
    highlightCurrentMatch(nextIndex);
  };

  const handlePrevMatch = () => {
    if (matches.length === 0) return;
    const prevIndex =
      currentMatchIndex === 0 ? matches.length - 1 : currentMatchIndex - 1;
    setCurrentMatchIndex(prevIndex);
    highlightCurrentMatch(prevIndex);
  };

  const clearSearch = () => {
    setSearchText("");
    clearMatches();
  };

  const handleDownload = () => {
    const blob = new Blob([svgContent], { type: "image/svg+xml" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = `diagram-${Date.now()}.svg`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  const handleZoomIn = () => {
    panzoomRef.current?.zoomIn({ force: true });
  };

  const handleZoomOut = () => {
    panzoomRef.current?.zoomOut({ force: true });
  };

  const handleReset = () => {
    panzoomRef.current?.reset({ force: true });
  };

  const handleMoveUp = () => {
    const pan = panzoomRef.current?.getPan();
    if (pan) {
      panzoomRef.current?.pan(pan.x, pan.y - 50, { force: true });
    }
  };

  const handleMoveDown = () => {
    const pan = panzoomRef.current?.getPan();
    if (pan) {
      panzoomRef.current?.pan(pan.x, pan.y + 50, { force: true });
    }
  };

  const handleMoveLeft = () => {
    const pan = panzoomRef.current?.getPan();
    if (pan) {
      panzoomRef.current?.pan(pan.x - 50, pan.y, { force: true });
    }
  };

  const handleMoveRight = () => {
    const pan = panzoomRef.current?.getPan();
    if (pan) {
      panzoomRef.current?.pan(pan.x + 50, pan.y, { force: true });
    }
  };

  const handleFitToView = () => {
    if (!containerRef.current || !panzoomRef.current) return;

    const parent = containerRef.current.parentElement;
    if (!parent) return;

    const svg = containerRef.current.querySelector("svg");
    if (!svg) return;

    const parentRect = parent.getBoundingClientRect();
    const svgRect = svg.getBoundingClientRect();

    const scaleX = parentRect.width / svgRect.width;
    const scaleY = parentRect.height / svgRect.height;
    const scale = Math.min(scaleX, scaleY) * 0.9;

    panzoomRef.current.zoom(scale, { animate: true, force: true });

    const scaledWidth = svgRect.width * scale;
    const scaledHeight = svgRect.height * scale;
    const x = (parentRect.width - scaledWidth) / 2;
    const y = (parentRect.height - scaledHeight) / 2;

    panzoomRef.current.pan(x, y, { animate: true, force: true });
  };

  useEffect(() => {
    return () => {
      clearAllHighlights();
    };
  }, []);

  useEffect(() => {
    if (searchText === "") {
      clearMatches();
    }
  }, [searchText]);

  return (
    <div className={cn("flex flex-col h-full relative", className)}>
      <div className="flex items-center gap-1 p-2 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 flex-wrap">
        <Button
          onClick={onHideEditor}
          size="sm"
          variant="ghost"
          title="Hide Editor"
        >
          <PenOff className="w-4 h-4" />
        </Button>

        <div className="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1" />

        <Button
          onClick={() => {
            setShowSearch(!showSearch);
            if (showSearch) {
              clearSearch();
            }
          }}
          size="sm"
          variant={showSearch ? "secondary" : "ghost"}
          title="Search"
        >
          <Search className="w-4 h-4" />
        </Button>

        <div className="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1" />

        <Button
          onClick={() => setMouseInteractionEnabled(!mouseInteractionEnabled)}
          size="sm"
          variant={mouseInteractionEnabled ? "secondary" : "ghost"}
          title={
            mouseInteractionEnabled
              ? "Disable Mouse Interaction"
              : "Enable Mouse Interaction"
          }
        >
          <Hand className="w-4 h-4" />
        </Button>

        <div className="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1" />

        <Button
          onClick={handleDownload}
          size="sm"
          variant="ghost"
          title="Download SVG"
        >
          <Download className="w-4 h-4" />
        </Button>

        {showSearch && (
          <>
            <div className="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1" />

            <div className="flex items-center gap-1">
              <div className="relative">
                <Input
                  type="text"
                  placeholder="Search text..."
                  value={searchText}
                  onChange={(e) => setSearchText(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === "Enter") {
                      if (e.shiftKey) {
                        handlePrevMatch();
                      } else {
                        handleSearch();
                      }
                    } else if (e.key === "Escape") {
                      clearSearch();
                    }
                  }}
                  className="h-8 w-48 pr-8"
                />
                {searchText && (
                  <button
                    type="button"
                    onClick={clearSearch}
                    className="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
                  >
                    <X className="w-4 h-4" />
                  </button>
                )}
              </div>

              <Button
                onClick={handleSearch}
                size="sm"
                variant="secondary"
                disabled={!searchText}
              >
                Find
              </Button>

              {matches.length > 0 && (
                <div className="flex items-center gap-1 ml-2">
                  <Button
                    onClick={handlePrevMatch}
                    size="sm"
                    variant="ghost"
                    title="Previous match (Shift+Enter)"
                  >
                    <ChevronLeft className="w-4 h-4" />
                  </Button>

                  <span className="text-sm text-gray-600 dark:text-gray-400 min-w-[60px] text-center">
                    {currentMatchIndex + 1} / {matches.length}
                  </span>

                  <Button
                    onClick={handleNextMatch}
                    size="sm"
                    variant="ghost"
                    title="Next match (Enter)"
                  >
                    <ChevronRight className="w-4 h-4" />
                  </Button>
                </div>
              )}

              {searchText && matches.length === 0 && (
                <span className="text-sm text-gray-500 ml-2">No matches</span>
              )}
            </div>
          </>
        )}
      </div>

      <div className="flex-1 overflow-hidden">
        <div
          ref={containerRef}
          className="w-full h-full flex items-center justify-center bg-white dark:bg-gray-900 p-8"
          dangerouslySetInnerHTML={{ __html: svgContent }}
        />
      </div>

      <div className="absolute bottom-4 right-4 flex flex-col gap-2 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg p-1">
        <div className="flex gap-1">
          <Button
            onClick={handleZoomIn}
            size="sm"
            variant="ghost"
            title="Zoom In"
            className="h-8 w-8 p-0"
          >
            <Plus className="w-4 h-4" />
          </Button>
          <Button
            onClick={handleZoomOut}
            size="sm"
            variant="ghost"
            title="Zoom Out"
            className="h-8 w-8 p-0"
          >
            <Minus className="w-4 h-4" />
          </Button>
        </div>

        <div className="grid grid-cols-3 gap-0.5">
          <div />
          <Button
            onClick={handleMoveUp}
            size="sm"
            variant="ghost"
            title="Move Up"
            className="h-8 w-8 p-0"
          >
            <ArrowUp className="w-4 h-4" />
          </Button>
          <div />
          <Button
            onClick={handleMoveLeft}
            size="sm"
            variant="ghost"
            title="Move Left"
            className="h-8 w-8 p-0"
          >
            <ArrowLeft className="w-4 h-4" />
          </Button>
          <Button
            onClick={handleReset}
            size="sm"
            variant="ghost"
            title="Reset View"
            className="h-8 w-8 p-0"
          >
            <RotateCcw className="w-4 h-4" />
          </Button>
          <Button
            onClick={handleMoveRight}
            size="sm"
            variant="ghost"
            title="Move Right"
            className="h-8 w-8 p-0"
          >
            <ArrowRight className="w-4 h-4" />
          </Button>
          <div />
          <Button
            onClick={handleMoveDown}
            size="sm"
            variant="ghost"
            title="Move Down"
            className="h-8 w-8 p-0"
          >
            <ArrowDown className="w-4 h-4" />
          </Button>
          <div />
        </div>

        <Button
          onClick={handleFitToView}
          size="sm"
          variant="ghost"
          title="Fit to View"
          className="h-8 w-full p-0"
        >
          <Maximize2 className="w-4 h-4" />
        </Button>
      </div>
    </div>
  );
};
