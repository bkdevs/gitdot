import baseVitesseDark from "@shikijs/themes/vitesse-dark";
import type { ThemeRegistrationRaw } from "shiki";

const base = baseVitesseDark as unknown as ThemeRegistrationRaw;

// Override the editor background so code blocks blend with the app's dark
// surface (--background = oklch(0.218 0 0) ≈ #1a1a1a in globals.css), and the
// default foreground so plain code text matches the app's --foreground
// (oklch(0.82 0 0) ≈ #c4c4c4) instead of vitesse's brighter warm off-white.
const theme: ThemeRegistrationRaw = {
  ...base,
  bg: "#1a1a1a",
  fg: "#c4c4c4",
  colors: {
    ...(base.colors ?? {}),
    "editor.background": "#1a1a1a",
    "editor.foreground": "#c4c4c4",
  },
};

export default theme;
