import { getSingletonHighlighter } from "shiki";
import gitdotLight from "./gitdot-light";

export async function loadGitdotLight() {
  const highlighter = await getSingletonHighlighter();
  if (!highlighter.getLoadedThemes().includes("gitdot-light")) {
    await highlighter.loadTheme(gitdotLight);
  }
  console.log(highlighter.getLoadedThemes());
}
