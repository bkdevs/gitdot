import baseVitesseDark from "@shikijs/themes/vitesse-dark";
import type { ThemeRegistrationRaw } from "shiki";

const base = baseVitesseDark as unknown as ThemeRegistrationRaw;

// Override the editor background so code blocks blend with the app's dark
// surface (--background = oklch(0.115 0 0) ≈ #050505 in globals.css).
const theme: ThemeRegistrationRaw = {
  ...base,
  bg: "#050505",
  colors: {
    ...(base.colors ?? {}),
    "editor.background": "#050505",
  },
};

export default theme;
