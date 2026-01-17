// Grayscale Light Theme
// Ported from base16-grayscale-light by Alexandre Gavioli
// https://github.com/chriskempson/base16-vim

// Base16 colors:
// 00: #f7f7f7 (background)
// 01: #e3e3e3 (lighter bg)
// 02: #b9b9b9 (selection)
// 03: #ababab (comments)
// 04: #525252 (dark fg)
// 05: #464646 (foreground)
// 06: #252525 (light fg)
// 07: #101010 (lightest fg)
// 08: #7c7c7c (variables, identifiers)
// 09: #999999 (constants, numbers)
// 0A: #a0a0a0 (types, classes)
// 0B: #8e8e8e (strings)
// 0C: #868686 (special, regex)
// 0D: #686868 (functions)
// 0E: #747474 (keywords)
// 0F: #5e5e5e (delimiters)

export const lightTheme = {
  name: "gitdot-light",
  displayName: "gitdot light",
  type: "light",
  colors: {
    "editor.background": "#f7f7f7",
    "editor.foreground": "#464646",
    "editor.selectionBackground": "#b9b9b9",
    "editor.inactiveSelectionBackground": "#e3e3e3",
    "editor.lineHighlightBackground": "#e3e3e3",
    "editorCursor.foreground": "#464646",
    "editorLineNumber.foreground": "#ababab",
    "editorLineNumber.activeForeground": "#525252",

    // Search - using gui09 for inc search, gui0A for search
    "editor.findMatchBackground": "#999999",
    "editor.findMatchHighlightBackground": "#a0a0a0",
  },
  semanticHighlighting: false,
  tokenColors: [
    // Default text - gui05
    {
      scope: ["source", "text"],
      settings: {
        foreground: "#464646",
      },
    },
    // Comments - gui03
    {
      scope: ["comment", "punctuation.definition.comment"],
      settings: {
        foreground: "#ababab",
        fontStyle: "italic",
      },
    },
    // Strings - gui0B
    {
      scope: ["string", "string.quoted", "string.template"],
      settings: {
        foreground: "#8e8e8e",
      },
    },
    // Constants, Numbers, Booleans - gui09
    {
      scope: [
        "constant",
        "constant.numeric",
        "constant.language",
        "constant.character",
      ],
      settings: {
        foreground: "#999999",
      },
    },
    // Variables, Identifiers - gui08
    {
      scope: [
        "variable",
        "variable.other",
        "variable.parameter",
        "entity.name.variable",
        "support.variable",
      ],
      settings: {
        foreground: "#7c7c7c",
      },
    },
    // Statements - gui08
    {
      scope: ["keyword.operator.expression", "variable.language"],
      settings: {
        foreground: "#7c7c7c",
      },
    },
    // Functions - gui0D
    {
      scope: [
        "entity.name.function",
        "support.function",
        "meta.function-call",
        "variable.function",
      ],
      settings: {
        foreground: "#686868",
      },
    },
    // Keywords, Conditionals, Control - gui0E
    {
      scope: [
        "keyword",
        "keyword.control",
        "keyword.other",
        "storage.modifier",
        "keyword.operator.new",
        "keyword.operator.logical",
      ],
      settings: {
        foreground: "#747474",
      },
    },
    // Operators - gui05 (same as normal text)
    {
      scope: ["keyword.operator", "punctuation"],
      settings: {
        foreground: "#464646",
      },
    },
    // Types, Classes, Storage - gui0A
    {
      scope: [
        "entity.name.type",
        "entity.name.class",
        "entity.name.namespace",
        "entity.name.struct",
        "support.type",
        "support.class",
        "storage.type",
      ],
      settings: {
        foreground: "#a0a0a0",
      },
    },
    // Special, Regex - gui0C
    {
      scope: [
        "string.regexp",
        "constant.other.symbol",
        "constant.other.key",
        "entity.other.attribute-name",
      ],
      settings: {
        foreground: "#868686",
      },
    },
    // Tags - gui08
    {
      scope: ["entity.name.tag", "punctuation.definition.tag"],
      settings: {
        foreground: "#7c7c7c",
      },
    },
    // Delimiters, Special chars - gui0F
    {
      scope: [
        "punctuation.separator",
        "punctuation.terminator",
        "meta.brace",
      ],
      settings: {
        foreground: "#5e5e5e",
      },
    },
    // Includes, Imports - gui0D
    {
      scope: [
        "keyword.control.import",
        "keyword.control.export",
        "keyword.control.from",
        "meta.preprocessor",
      ],
      settings: {
        foreground: "#686868",
      },
    },
    // Labels, PreProc - gui0A
    {
      scope: ["entity.name.label", "keyword.control.directive"],
      settings: {
        foreground: "#a0a0a0",
      },
    },
    // Error - gui08 on gui00
    {
      scope: ["invalid", "invalid.illegal"],
      settings: {
        foreground: "#f7f7f7",
        background: "#7c7c7c",
      },
    },
    // Diff - using appropriate grays
    {
      scope: ["markup.inserted", "meta.diff.header.to-file"],
      settings: {
        foreground: "#8e8e8e",
      },
    },
    {
      scope: ["markup.deleted", "meta.diff.header.from-file"],
      settings: {
        foreground: "#7c7c7c",
      },
    },
    {
      scope: ["markup.changed"],
      settings: {
        foreground: "#686868",
      },
    },
    // Markdown
    {
      scope: ["markup.heading", "entity.name.section"],
      settings: {
        foreground: "#686868",
        fontStyle: "bold",
      },
    },
    {
      scope: ["markup.bold"],
      settings: {
        fontStyle: "bold",
      },
    },
    {
      scope: ["markup.italic"],
      settings: {
        fontStyle: "italic",
      },
    },
    {
      scope: ["markup.inline.raw", "markup.raw"],
      settings: {
        foreground: "#8e8e8e",
      },
    },
  ],
};
