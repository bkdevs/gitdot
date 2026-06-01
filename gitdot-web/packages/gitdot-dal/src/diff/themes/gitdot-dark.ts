import baseAyuDark from "@shikijs/themes/ayu-dark";
import type { ThemeRegistrationRaw } from "shiki";

const base = baseAyuDark as unknown as ThemeRegistrationRaw;

// gitdot's dark theme: ayu-dark with the editor background pinned to the app's
// dark surface (--background = oklch(0.218 0 0) ≈ #1a1a1a in globals.css) and
// the foreground to the app's --foreground (oklch(0.82 0 0) ≈ #c4c4c4).
//
// Both bg AND fg must be set: shiki's normalizeTheme only keeps these when both
// are present — if either is missing it re-derives both from the theme's global
// token setting, clobbering the override.
const theme: ThemeRegistrationRaw = {
  ...base,
  name: "gitdot-dark",
  bg: "#1a1a1a",
  fg: "#c4c4c4",
  colors: {
    ...(base.colors ?? {}),
    "editor.background": "#1a1a1a",
    "editor.foreground": "#c4c4c4",
  },
};

export default theme;
