/// <reference lib="webworker" />

import { type BundledLanguage, getSingletonHighlighter } from "shiki";

type Theme = "vitesse-light" | "gitdot-light";

interface WorkerMessage {
  code: string;
  lang: string;
  theme: Theme;
}

self.onmessage = async function (event: MessageEvent<WorkerMessage>) {
  const { code, lang, theme } = event.data;
  const highlighter = await getSingletonHighlighter();

  if (!highlighter.getLoadedThemes().includes(theme)) {
    if (theme === "gitdot-light") {
      const gitdotLight = (await import("@/themes/gitdot-light")).default;
      await highlighter.loadTheme(gitdotLight);
    } else {
      const vitesseLight = (await import("@shikijs/themes/vitesse-light")).default;
      await highlighter.loadTheme(vitesseLight);
    }
  }

  if (!highlighter.getLoadedLanguages().includes(lang as BundledLanguage)) {
    await highlighter.loadLanguage(lang as BundledLanguage);
  }

  const html = highlighter.codeToHtml(code, { lang, theme });
  self.postMessage({ html });
};
