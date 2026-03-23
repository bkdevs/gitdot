import angularHtml from "@shikijs/langs/angular-html";
import angularTs from "@shikijs/langs/angular-ts";
import bash from "@shikijs/langs/bash";
import c from "@shikijs/langs/c";
import clojure from "@shikijs/langs/clojure";
import commonLisp from "@shikijs/langs/common-lisp";
import cpp from "@shikijs/langs/cpp";
import csharp from "@shikijs/langs/csharp";
import css from "@shikijs/langs/css";
import dart from "@shikijs/langs/dart";
import diff from "@shikijs/langs/diff";
import dockerfile from "@shikijs/langs/dockerfile";
import dotenv from "@shikijs/langs/dotenv";
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
import vitesseLight from "@shikijs/themes/vitesse-light";
import type { BundledLanguage } from "shiki";
import { createHighlighterCore } from "shiki/core";
import { createOnigurumaEngine } from "shiki/engine/oniguruma";
import gitdotLight from "@/themes/gitdot-light";

export function createHighlighter() {
  return createHighlighterCore({
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
      dotenv,
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
    engine: createOnigurumaEngine(import("shiki/wasm")),
  });
}

export function inferLanguage(filePath: string): BundledLanguage | undefined {
  const extension = filePath.split(".").pop()?.toLowerCase();
  const fileName = filePath.split("/").pop()?.toLowerCase();

  if (fileName === "dockerfile") return "dockerfile";
  if (fileName === "makefile") return "makefile";
  if (fileName === "codeowners") return "codeowners";
  if (fileName === ".env" || fileName?.startsWith(".env.")) return "dotenv";

  const extensionMap: Record<string, BundledLanguage> = {
    ts: "typescript",
    tsx: "tsx",
    js: "javascript",
    jsx: "jsx",
    mjs: "mjs",
    cjs: "cjs",
    py: "python",
    rs: "rust",
    go: "go",
    java: "java",
    c: "c",
    cpp: "cpp",
    cc: "cpp",
    cxx: "cpp",
    h: "c",
    hpp: "cpp",
    cs: "csharp",
    rb: "ruby",
    php: "php",
    swift: "swift",
    kt: "kotlin",
    kts: "kts",
    scala: "scala",
    r: "r",
    dart: "dart",
    lua: "lua",
    sql: "sql",
    html: "html",
    css: "css",
    scss: "scss",
    sass: "sass",
    less: "less",
    json: "json",
    jsonc: "jsonc",
    json5: "json5",
    yaml: "yaml",
    yml: "yml",
    toml: "toml",
    xml: "xml",
    md: "markdown",
    mdx: "mdx",
    sh: "bash",
    bash: "bash",
    zsh: "zsh",
    fish: "fish",
    ps1: "powershell",
    bat: "batch",
    cmd: "cmd",
    vue: "vue",
    svelte: "svelte",
    astro: "astro",
    elm: "elm",
    erl: "erlang",
    ex: "elixir",
    exs: "elixir",
    fs: "fsharp",
    hs: "haskell",
    clj: "clojure",
    coffee: "coffeescript",
    nim: "nim",
    v: "v",
    zig: "zig",
    graphql: "graphql",
    gql: "graphql",
    proto: "protobuf",
    tf: "terraform",
    tfvars: "tfvars",
    hcl: "hcl",
    dockerfile: "dockerfile",
    tex: "latex",
    vim: "vim",
    asm: "asm",
    sol: "solidity",
    vy: "vyper",
    move: "move",
    cairo: "cairo",
    prisma: "prisma",
    adoc: "asciidoc",
    rst: "rst",
    diff: "diff",
    csv: "csv",
    tsv: "tsv",
  };

  return extension ? extensionMap[extension] : undefined;
}
