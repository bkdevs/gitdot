import bash from "@shikijs/langs/bash";
import c from "@shikijs/langs/c";
import cpp from "@shikijs/langs/cpp";
import css from "@shikijs/langs/css";
import go from "@shikijs/langs/go";
import html from "@shikijs/langs/html";
import java from "@shikijs/langs/java";
import javascript from "@shikijs/langs/javascript";
import json from "@shikijs/langs/json";
import python from "@shikijs/langs/python";
import rust from "@shikijs/langs/rust";
import sql from "@shikijs/langs/sql";
import toml from "@shikijs/langs/toml";
import tsx from "@shikijs/langs/tsx";
import typescript from "@shikijs/langs/typescript";
import yaml from "@shikijs/langs/yaml";
import vitesseLight from "@shikijs/themes/vitesse-light";
import { addClassToHast, type ThemeRegistrationRaw } from "shiki";
import { createHighlighterCoreSync } from "shiki/core";
import { createJavaScriptRegexEngine } from "shiki/engine/javascript";

const highlighter = createHighlighterCoreSync({
  themes: [vitesseLight as unknown as ThemeRegistrationRaw],
  langs: [
    bash,
    c,
    cpp,
    css,
    go,
    html,
    java,
    javascript,
    json,
    python,
    rust,
    sql,
    toml,
    tsx,
    typescript,
    yaml,
  ],
  engine: createJavaScriptRegexEngine(),
});

export function highlightMarkdownCode(
  code: string,
  lang: string,
): string | null {
  if (!lang) return null;
  try {
    return highlighter.codeToHtml(code, {
      lang,
      theme: "vitesse-light",
      transformers: [
        {
          pre(node) {
            addClassToHast(node, "rounded p-4 mb-4 overflow-x-auto");
            node.properties.style = `${node.properties.style ?? ""}; background-color: #f7f7f7; font-family: ui-monospace, 'Cascadia Code', 'Fira Code', Menlo, Consolas, monospace; font-size: 0.875rem;`;
          },
        },
      ],
    });
  } catch {
    return null;
  }
}
