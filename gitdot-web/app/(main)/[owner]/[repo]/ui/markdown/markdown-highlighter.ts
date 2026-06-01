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
import { gitdotDark, gitdotLight } from "gitdot-dal/client";
import { addClassToHast } from "shiki";
import { createHighlighterCoreSync } from "shiki/core";
import { createJavaScriptRegexEngine } from "shiki/engine/javascript";

const highlighter = createHighlighterCoreSync({
  themes: [gitdotLight, gitdotDark],
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
      themes: { light: "gitdot-light", dark: "gitdot-dark" },
      defaultColor: "light",
      transformers: [
        {
          pre(node) {
            addClassToHast(
              node,
              "markdown-shiki rounded p-4 mb-4 overflow-x-auto",
            );
            node.properties.style = `${node.properties.style ?? ""}; font-family: ui-monospace, 'Cascadia Code', 'Fira Code', Menlo, Consolas, monospace; font-size: 0.875rem;`;
          },
        },
      ],
    });
  } catch {
    return null;
  }
}
