import {
  ChevronLeft,
  ChevronRight,
  Download,
  Maximize2,
  Minus,
  PenOff,
  Plus,
  RotateCcw,
  Search,
  X,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { TransformComponent, TransformWrapper } from "react-zoom-pan-pinch";
import { Button } from "@/common/component/ui/button";
import { Input } from "@/common/component/ui/input";
import { cn } from "@/lib/utils";

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
  const containerRef = useRef<HTMLDivElement>(null);
  const matchesRef = useRef<MatchInfo[]>([]);

  useEffect(() => {
    matchesRef.current = matches;
  }, [matches]);

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
    <TransformWrapper
      initialScale={1}
      minScale={0.1}
      maxScale={10}
      centerOnInit
      limitToBounds={false}
    >
      {({ zoomIn, zoomOut, resetTransform, centerView }) => (
        <div className={cn("flex flex-col h-full", className)}>
          <div className="flex items-center gap-1 p-2 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 flex-wrap">
            <Button
              onClick={onHideEditor}
              size="sm"
              variant="ghost"
              title="Zoom In"
            >
              <PenOff className="w-4 h-4" />
            </Button>
            <Button
              onClick={() => zoomIn()}
              size="sm"
              variant="ghost"
              title="Zoom In"
            >
              <Plus className="w-4 h-4" />
            </Button>
            <Button
              onClick={() => zoomOut()}
              size="sm"
              variant="ghost"
              title="Zoom Out"
            >
              <Minus className="w-4 h-4" />
            </Button>
            <Button
              onClick={() => resetTransform()}
              size="sm"
              variant="ghost"
              title="Reset View"
            >
              <RotateCcw className="w-4 h-4" />
            </Button>
            <Button
              onClick={() => centerView()}
              size="sm"
              variant="ghost"
              title="Fit to View"
            >
              <Maximize2 className="w-4 h-4" />
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
                    <span className="text-sm text-gray-500 ml-2">
                      No matches
                    </span>
                  )}
                </div>
              </>
            )}
          </div>

          <div className="flex-1 overflow-hidden">
            <TransformComponent
              wrapperStyle={{
                width: "100%",
                height: "100%",
              }}
              contentStyle={{
                width: "100%",
                height: "100%",
              }}
            >
              <div
                ref={containerRef}
                className="w-full h-full flex items-center justify-center bg-white dark:bg-gray-900 p-8"
                dangerouslySetInnerHTML={{ __html: svgContent }}
              />
            </TransformComponent>
          </div>
        </div>
      )}
    </TransformWrapper>
  );
};
