import baseVitesseLight from "@shikijs/themes/vitesse-light";
import type { ThemeRegistrationRaw } from "shiki";

const base = baseVitesseLight as unknown as ThemeRegistrationRaw;

// gitdot's light theme: vitesse-light with the editor background pinned to the
// app's light surface (--background = oklch(1 0 0) ≈ #ffffff in globals.css),
// keeping vitesse-light's own foreground (#393a34).
//
// Both bg AND fg must be set: shiki's normalizeTheme only keeps these when both
// are present — if either is missing it re-derives both from the theme's global
// token setting, clobbering the override.
const theme: ThemeRegistrationRaw = {
  ...base,
  name: "gitdot-light",
  bg: "#ffffff",
  fg: "#393a34",
  colors: {
    ...(base.colors ?? {}),
    "editor.background": "#ffffff",
    "editor.foreground": "#393a34",
  },
};

export default theme;
