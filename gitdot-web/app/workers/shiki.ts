/// <reference lib="webworker" />

// module imports to discourage chunk splitting
import angularHtml from "@shikijs/langs/angular-html";
import angularTs from "@shikijs/langs/angular-ts";
import bash from "@shikijs/langs/bash";
import c from "@shikijs/langs/c";
import clojure from "@shikijs/langs/clojure";
import commonLisp from "@shikijs/langs/common-lisp";
import cpp from "@shikijs/langs/cpp";
import css from "@shikijs/langs/css";
import csharp from "@shikijs/langs/csharp";
import dart from "@shikijs/langs/dart";
import diff from "@shikijs/langs/diff";
import dockerfile from "@shikijs/langs/dockerfile";
import elixir from "@shikijs/langs/elixir";
import emacsLisp from "@shikijs/langs/emacs-lisp";
import go from "@shikijs/langs/go";
import graphql from "@shikijs/langs/graphql";
import haskell from "@shikijs/langs/haskell";
import hcl from "@shikijs/langs/hcl";
import html from "@shikijs/langs/html";
import java from "@shikijs/langs/java";
import javascript from "@shikijs/langs/javascript";
import json from "@shikijs/langs/json";
import jsonc from "@shikijs/langs/jsonc";
import jsx from "@shikijs/langs/jsx";
import kotlin from "@shikijs/langs/kotlin";
import makefile from "@shikijs/langs/makefile";
import markdown from "@shikijs/langs/markdown";
import nginx from "@shikijs/langs/nginx";
import ocaml from "@shikijs/langs/ocaml";
import php from "@shikijs/langs/php";
import python from "@shikijs/langs/python";
import ruby from "@shikijs/langs/ruby";
import rust from "@shikijs/langs/rust";
import scala from "@shikijs/langs/scala";
import scss from "@shikijs/langs/scss";
import sql from "@shikijs/langs/sql";
import svelte from "@shikijs/langs/svelte";
import swift from "@shikijs/langs/swift";
import terraform from "@shikijs/langs/terraform";
import toml from "@shikijs/langs/toml";
import tsx from "@shikijs/langs/tsx";
import typescript from "@shikijs/langs/typescript";
import vue from "@shikijs/langs/vue";
import xml from "@shikijs/langs/xml";
import yaml from "@shikijs/langs/yaml";
import zig from "@shikijs/langs/zig";

import gitdotLight from "@/themes/gitdot-light";
import vitesseLight from "@shikijs/themes/vitesse-light";

import { createHighlighterCore } from 'shiki/core'
import { createOnigurumaEngine } from 'shiki/engine/oniguruma'

const highlighter = await createHighlighterCore({
  langs: [
    angularHtml,
    angularTs,
    bash,
    c,
    clojure,
    commonLisp,
    cpp,
    css,
    csharp,
    dart,
    diff,
    dockerfile,
    elixir,
    emacsLisp,
    go,
    graphql,
    haskell,
    hcl,
    html,
    java,
    javascript,
    json,
    jsonc,
    jsx,
    kotlin,
    makefile,
    markdown,
    nginx,
    ocaml,
    php,
    python,
    ruby,
    rust,
    scala,
    scss,
    sql,
    svelte,
    swift,
    terraform,
    toml,
    tsx,
    typescript,
    vue,
    xml,
    yaml,
    zig,
  ],
  themes: [vitesseLight, gitdotLight],
  engine: createOnigurumaEngine(import("shiki/wasm"))
})

interface WorkerMessage {
  code: string;
  lang: string;
  theme: "vitesse-light" | "gitdot-light";
}

self.onmessage = async function (event: MessageEvent<WorkerMessage>) {
  const { code, lang, theme } = event.data;
  const html = highlighter.codeToHtml(code, { lang, theme });
  self.postMessage({ html });
};
