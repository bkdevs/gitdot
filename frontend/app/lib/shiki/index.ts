import {
  bundledLanguages,
  bundledThemes,
  createHighlighter,
} from "shiki/bundle/full";
import { lightTheme } from "./light";

export const highlighter = await createHighlighter({
  themes: [lightTheme, ...Object.keys(bundledThemes)],
  langs: Object.keys(bundledLanguages), // TODO: do this lazily?
});
