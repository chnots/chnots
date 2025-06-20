import { EditorView } from "@uiw/react-codemirror";

export const createCodemirrorTheme = () => {
  const isDarkTheme = false;
  const editorNoGuttersSelector = "&:not(:has(> .cm-scroller > .cm-gutters))";
  const theme = {
    fontFamily:
      "IBM Plex Mono, monospace, IBM Plex Sans SC, Helvetica Neue, Arial, PingFang SC, Hiragino Sans GB, Microsoft YaHei, sans-serif",
    fontSize: 14,
    fontSizeUnits: undefined,
    isDesktop: true,
    marginLeft: 0,
    marginRight: 0,
    colorFaded: "#999",
    listTabSize: 2,
    blockQuoteOpacity: "0.5",
    searchMarkerColor: "black",
    searchMarkerBackgroundColor: "white",
  };
  const monospaceStyle: Record<string, string> = {};
  const baseGlobalStyle: Record<string, string> = {
    color: "#222",
    backgroundColor: "#fff",

    // On iOS, apply system font scaling (e.g. font scaling
    // set in accessibility settings).
    font: "-apple-system-body",

    // Fill container horizontally
    width: "100%",
    boxSizing: "border-box",
  };
  const baseCursorStyle: Record<string, string> = {};
  const baseSelectionStyle: Record<string, string> = {};
  const blurredSelectionStyle: Record<string, string> = {};

  const baseContentStyle: Record<string, string | undefined> = {
    fontFamily: theme.fontFamily,
    fontSize: `${theme.fontSize}${theme.fontSizeUnits ?? "px"}`,

    // Avoid using units here -- 1.55em, for example, can cause lines to overlap
    // if some lines contain text with a large enough font size.
    lineHeight: theme.isDesktop ? "1.55" : undefined,
  };
  const baseHeadingStyle = {
    fontWeight: "bold",
    fontFamily: theme.fontFamily,
  };
  return EditorView.theme({
    "&.cm-editor": {
      background: "transparent !important",
    },
    ".cm-content": {
      padding: "0em",
    },
    ".cm-lineWrapping": {
      wordBreak: "break-all",
    },
    ".hashtag": {
      border: "1px solid #602533",
      padding: "1px",
      borderRadius: "0.2em",
      color: "#682d4b",
    },
    // Include &.CodeMirror to handle the case where additional CodeMirror 5 styles
    // need to be overridden.
    "&, &.CodeMirror": baseGlobalStyle,

    "& .cm-dropCursor": {
      backgroundColor: isDarkTheme ? "white" : "black",
      width: "1px",
    },

    // These must be !important or more specific than CodeMirror's built-ins
    "& .cm-content": {
      fontFamily: theme.fontFamily,
      ...baseContentStyle,
      paddingBottom: theme.isDesktop ? "400px" : null,
      marginLeft: `${theme.marginLeft}px`,
      marginRight: `${theme.marginRight}px`,
    },

    "& .cm-listItem": {
      // Needs to be !important because the tab-size is directly set on the element style
      // attribute by CodeMirror. And the `EditorState.tabSize` function only accepts a
      // number, while we need a "em" value to make it match the viewer tab size.
      tabSize: `${theme.listTabSize} !important`,
    },

    "&.cm-focused .cm-cursor": baseCursorStyle,

    // The desktop app sets the font for these elements to a specific font.
    // Override this.
    "& div, & span, & a": {
      fontFamily: "inherit",
    },

    // Override the default border around CodeMirror panels
    "& > .cm-panels": {
      border: "none",
    },

    // &.cm-focused is used to give these styles higher specificity
    // than the defaults.
    // [selectionBackgroundSelector]: baseSelectionStyle,
    "&.cm-focused ::selection": baseSelectionStyle,
    "& ::selection": blurredSelectionStyle,
    "& .cm-selectionLayer .cm-selectionBackground": blurredSelectionStyle,

    "&.cm-editor.cm-focused": {
      outline: "none !important",
    },

    "& .cm-blockQuote": {
      borderLeft: `4px solid ${theme.colorFaded}`,
      opacity: theme.blockQuoteOpacity,
      paddingLeft: "4px",
    },

    "& .cm-codeBlock": {
      "&.cm-regionFirstLine, &.cm-regionLastLine": {
        borderRadius: "2px",
      },
      "&:not(.cm-regionFirstLine)": {
        borderTop: "none",
        borderTopLeftRadius: 0,
        borderTopRightRadius: 0,
      },
      "&:not(.cm-regionLastLine)": {
        borderBottom: "none",
        borderBottomLeftRadius: 0,
        borderBottomRightRadius: 0,
      },

      borderWidth: "1px",
      borderStyle: "solid",
      borderColor: theme.colorFaded,
      backgroundColor: "rgba(155, 155, 155, 0.1)",

      ...monospaceStyle,
    },

    // CodeMirror wraps the existing inline span in an additional element.
    // Due to a Chrome rendering bug, because the .cm-inlineCode wraps a
    // span with a larger font-size, the .cm-inlineCode's bounding box won't
    // be big enough for its content.
    // As such, we need to style whichever element directly wraps its content.
    "& .cm-inlineCode": {
      borderWidth: "1px",
      borderStyle: "solid",
      borderColor: isDarkTheme
        ? "rgba(200, 200, 200, 0.5)"
        : "rgba(100, 100, 100, 0.5)",
      borderRadius: "4px",

      ...monospaceStyle,
    },

    "& .cm-mathBlock, & .cm-inlineMath": {
      color: isDarkTheme ? "#9fa" : "#276",
    },

    "& .cm-tableHeader, & .cm-tableRow, & .cm-tableDelimiter": monospaceStyle,
    "& .cm-taskMarker": monospaceStyle,

    // Allows editor content to be left-aligned with the toolbar on desktop.
    // See https://github.com/laurent22/joplin/issues/11279
    [`${editorNoGuttersSelector} .cm-line`]: theme.isDesktop
      ? {
          // Note: This cannot be zero:
          paddingLeft: "1px",
        }
      : {},

    // Override the default URL style when the URL is within a link
    "& .tok-url.tok-link, & .tok-link.tok-meta, & .tok-link.tok-string": {
      opacity: 0.661,
    },

    "& .cm-strike": {
      textDecoration: "line-through",
    },

    // Applying font size changes with CSS rather than the theme below works
    // around an issue where the border for code blocks in headings was too
    // small.
    "& .cm-h1": {
      ...baseHeadingStyle,
      fontSize: "1.6em",
    },
    "& .cm-h2": {
      ...baseHeadingStyle,
      fontSize: "1.4em",
    },
    "& .cm-h3": {
      ...baseHeadingStyle,
      fontSize: "1.3em",
    },
    "& .cm-h4": {
      ...baseHeadingStyle,
      fontSize: "1.2em",
    },
    "& .cm-h5": {
      ...baseHeadingStyle,
      fontSize: "1.1em",
    },
    "& .cm-h6": {
      ...baseHeadingStyle,
      fontSize: "1.0em",
    },

    "& .cm-highlighted": {
      color: theme.searchMarkerColor,
      backgroundColor: theme.searchMarkerBackgroundColor,
    },

    // Style the search widget. Use ':root' to increase the selector's precedence
    // (override the existing preset styles).
    ":root & .cm-panel.cm-search": {
      "& label, & button, & input": {
        fontSize: "1em",
        color: isDarkTheme ? "white" : "black",
      },
    },
  });
};
