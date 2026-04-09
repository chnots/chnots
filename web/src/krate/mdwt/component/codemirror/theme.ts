import { EditorView } from "@codemirror/view";

export const createCodemirrorTheme = (isDarkTheme = false) => {
  const editorNoGuttersSelector = "&:not(:has(> .cm-scroller > .cm-gutters))";

  // shadcn/ui color palette (oklch values converted to approximate hex for CodeMirror)
  const colors = isDarkTheme
    ? {
        background: "#242424",
        foreground: "#fafafa",
        muted: "#2a2a2a",
        mutedForeground: "#a1a1a1",
        border: "#363636",
        accent: "#2a2a2a",
        accentForeground: "#fafafa",
        primary: "#fafafa",
        primaryForeground: "#171717",
        ring: "#525252",
        destructive: "#7f1d1d",
      }
    : {
        background: "#ffffff",
        foreground: "#171717",
        muted: "#f5f5f5",
        mutedForeground: "#737373",
        border: "#e5e5e5",
        accent: "#f5f5f5",
        accentForeground: "#171717",
        primary: "#171717",
        primaryForeground: "#fafafa",
        ring: "#a3a3a3",
        destructive: "#dc2626",
      };

  const theme = {
    fontFamily:
      'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace',
    fontSize: 14,
    fontSizeUnits: undefined,
    isDesktop: true,
    marginLeft: 0,
    marginRight: 0,
    colorFaded: colors.mutedForeground,
    listTabSize: 2,
    blockQuoteOpacity: "0.7",
    searchMarkerColor: colors.primaryForeground,
    searchMarkerBackgroundColor: colors.primary,
  };

  const monospaceStyle: Record<string, string> = {
    fontFamily: theme.fontFamily,
  };

  const baseGlobalStyle: Record<string, string> = {
    color: colors.foreground,
    backgroundColor: "transparent",
    font: "-apple-system-body",
    width: "100%",
    boxSizing: "border-box",
  };

  const baseCursorStyle: Record<string, string> = {
    borderLeftColor: colors.primary,
    borderLeftWidth: "2px",
  };

  const baseSelectionStyle: Record<string, string> = {
    backgroundColor: `${colors.primary}33`,
  };

  const blurredSelectionStyle: Record<string, string> = {
    backgroundColor: `${colors.mutedForeground}26`,
  };

  const baseContentStyle: Record<string, string | undefined> = {
    fontFamily: theme.fontFamily,
    fontSize: `${theme.fontSize}${theme.fontSizeUnits ?? "px"}`,
    lineHeight: theme.isDesktop ? "1.6" : undefined,
    color: colors.foreground,
  };

  const baseHeadingStyle = {
    fontWeight: "600",
    fontFamily:
      'ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    color: colors.foreground,
    letterSpacing: "-0.025em",
  };

  return EditorView.theme({
    "&.cm-editor": {
      background: "transparent !important",
    },

    ".cm-content": {
      padding: "0.5rem 0",
      caretColor: colors.primary,
    },

    ".cm-lineWrapping": {
      wordBreak: "break-word",
      overflowWrap: "break-word",
    },

    "&, &.CodeMirror": baseGlobalStyle,

    ".cm-activeLine": {
      backgroundColor: `${colors.muted}80`,
      borderRadius: "4px",
    },

    "& .cm-dropCursor": {
      backgroundColor: colors.primary,
      width: "2px",
    },

    "& .cm-content": {
      ...baseContentStyle,
      paddingBottom: theme.isDesktop ? "1rem" : "0.5rem",
      marginLeft: `${theme.marginLeft}px`,
      marginRight: `${theme.marginRight}px`,
    },

    "& .cm-listItem": {
      tabSize: `${theme.listTabSize} !important`,
    },

    "&.cm-focused .cm-cursor": baseCursorStyle,

    "& div, & span, & a": {
      fontFamily: "inherit",
    },

    "& > .cm-panels": {
      border: `1px solid ${colors.border}`,
      borderRadius: "0.5rem",
      backgroundColor: colors.background,
      margin: "0.5rem",
      boxShadow: "0 1px 3px 0 rgb(0 0 0 / 0.1)",
    },

    "&.cm-focused ::selection": baseSelectionStyle,
    "& ::selection": blurredSelectionStyle,
    "& .cm-selectionLayer .cm-selectionBackground": blurredSelectionStyle,

    "&.cm-editor.cm-focused": {
      outline: "none !important",
    },

    "& .cm-blockQuote": {
      borderLeft: `3px solid ${colors.border}`,
      opacity: theme.blockQuoteOpacity,
      paddingLeft: "1rem",
      marginLeft: "0.5rem",
      color: colors.mutedForeground,
      fontStyle: "italic",
    },

    "& .cm-codeBlock": {
      "&.cm-regionFirstLine, &.cm-regionLastLine": {
        borderRadius: "0.375rem",
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
      borderColor: colors.border,
      backgroundColor: colors.muted,
      ...monospaceStyle,
      fontSize: "0.875em",
      padding: "0.25rem 0.5rem",
    },

    "& .cm-inlineCode": {
      borderWidth: "1px",
      borderStyle: "solid",
      borderColor: colors.border,
      borderRadius: "0.25rem",
      backgroundColor: colors.muted,
      padding: "0.125rem 0.375rem",
      fontSize: "0.875em",
      ...monospaceStyle,
    },

    "& .cm-mathBlock, & .cm-inlineMath": {
      color: isDarkTheme ? "#7dd3fc" : "#0369a1",
      fontFamily: theme.fontFamily,
    },

    "& .cm-tableHeader, & .cm-tableRow, & .cm-tableDelimiter": monospaceStyle,
    "& .cm-taskMarker": monospaceStyle,

    [`${editorNoGuttersSelector} .cm-line`]: theme.isDesktop
      ? {
          paddingLeft: "4px",
          paddingRight: "4px",
        }
      : {},

    "& .tok-url.tok-link, & .tok-link.tok-meta, & .tok-link.tok-string": {
      color: isDarkTheme ? "#7dd3fc" : "#0369a1",
      textDecoration: "underline",
      textUnderlineOffset: "2px",
    },

    "& .cm-strike": {
      textDecoration: "line-through",
      textDecorationColor: colors.mutedForeground,
      opacity: 0.8,
    },

    "& .cm-h1": {
      ...baseHeadingStyle,
      fontSize: "1.5em",
    },
    "& .cm-h2": {
      ...baseHeadingStyle,
      fontSize: "1.25em",
    },
    "& .cm-h3": {
      ...baseHeadingStyle,
      fontSize: "1.125em",
    },
    "& .cm-h4": {
      ...baseHeadingStyle,
      fontSize: "1.05em",
    },
    "& .cm-h5": {
      ...baseHeadingStyle,
      fontSize: "1em",
    },
    "& .cm-h6": {
      ...baseHeadingStyle,
      fontSize: "0.95em",
      color: colors.mutedForeground,
    },

    "& .cm-highlighted": {
      color: theme.searchMarkerColor,
      backgroundColor: theme.searchMarkerBackgroundColor,
      borderRadius: "2px",
      padding: "0 2px",
    },

    ":root & .cm-panel.cm-search": {
      padding: "0.75rem",
      "& label, & button, & input": {
        fontSize: "0.875rem",
        color: colors.foreground,
        fontFamily:
          'ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
      },
      "& input": {
        border: `1px solid ${colors.border}`,
        borderRadius: "0.375rem",
        padding: "0.375rem 0.75rem",
        backgroundColor: colors.background,
        outline: "none",
        "&:focus": {
          borderColor: colors.ring,
          boxShadow: `0 0 0 2px ${colors.ring}40`,
        },
      },
      "& button": {
        border: `1px solid ${colors.border}`,
        borderRadius: "0.375rem",
        padding: "0.375rem 0.75rem",
        backgroundColor: colors.muted,
        cursor: "pointer",
        transition: "all 0.15s ease",
        "&:hover": {
          backgroundColor: colors.accent,
        },
        "&:active": {
          backgroundColor: colors.border,
        },
      },
    },

    ".cm-hashtag-mark, .cm-hashtag-label": {
      color: isDarkTheme ? "#f0abfc" : "#a21caf",
      fontWeight: "500",
    },

    ".cm-backlink-mark, .cm-backlink-id": {
      color: isDarkTheme ? "#5eead4" : "#0d9488",
      fontWeight: "500",
    },

    ".cm-toent-mark": {
      color: colors.destructive,
      fontWeight: "500",
    },

    ".cm-todo-highlight": {
      color: colors.destructive,
      fontWeight: "600",
    },

    // Line numbers styling
    "& .cm-gutters": {
      backgroundColor: "transparent",
      borderRight: `1px solid ${colors.border}`,
      color: colors.mutedForeground,
      fontFamily: theme.fontFamily,
      fontSize: "0.75rem",
      paddingRight: "0.5rem",
    },

    "& .cm-activeLineGutter": {
      backgroundColor: `${colors.muted}80`,
      color: colors.foreground,
    },

    // Horizontal rule
    "& .cm-horizontalRule": {
      borderTop: `1px solid ${colors.border}`,
      marginTop: "1rem",
      marginBottom: "1rem",
    },

    // Live preview theme overrides
    "& .cm-livePreview-hr": {
      borderTop: `1px solid ${colors.border}`,
    },
    "& .cm-livePreview-image-error": {
      color: colors.mutedForeground,
    },
    "& .cm-livePreview-math": {
      color: isDarkTheme ? "#7dd3fc" : "#0369a1",
    },

    // Lists
    "& .cm-list": {
      paddingLeft: "1.5rem",
    },

    // Emphasis
    "& .cm-emphasis": {
      fontStyle: "italic",
    },

    "& .cm-strong": {
      fontWeight: "600",
    },

    // Comments (for code blocks)
    "& .tok-comment": {
      color: colors.mutedForeground,
      fontStyle: "italic",
    },

    "& .tok-keyword": {
      color: isDarkTheme ? "#fca5a5" : "#dc2626",
      fontWeight: "500",
    },

    "& .tok-string": {
      color: isDarkTheme ? "#86efac" : "#16a34a",
    },

    "& .tok-number": {
      color: isDarkTheme ? "#fdba74" : "#ea580c",
    },

    "& .tok-function": {
      color: isDarkTheme ? "#93c5fd" : "#2563eb",
    },

    "& .tok-typeName": {
      color: isDarkTheme ? "#c4b5fd" : "#7c3aed",
    },

    // Focus ring for accessibility
    "&.cm-focused": {
      outline: "none",
    },
  });
};
