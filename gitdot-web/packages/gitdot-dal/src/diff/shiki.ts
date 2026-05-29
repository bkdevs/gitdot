import type { Element, ElementContent, Root } from "hast";
import {
  type BundledLanguage,
  getSingletonHighlighter,
  type Highlighter,
  type ShikiTransformer,
} from "shiki";

async function getHighlighter(
  lang: BundledLanguage | undefined,
): Promise<Highlighter> {
  const highlighter = await getSingletonHighlighter();
  const loaded = highlighter.getLoadedThemes();

  if (!loaded.includes("vitesse-light")) {
    await highlighter.loadTheme(
      (await import("@shikijs/themes/vitesse-light")).default,
    );
  }
  if (!loaded.includes("vitesse-dark")) {
    await highlighter.loadTheme(
      (await import("../themes/vitesse-dark")).default,
    );
  }
  if (!loaded.includes("gitdot-light")) {
    await highlighter.loadTheme(
      (await import("../themes/gitdot-light")).default,
    );
  }

  if (lang && !highlighter.getLoadedLanguages().includes(lang)) {
    await highlighter.loadLanguage(lang);
  }
  return highlighter;
}

/*
 * turns a plaintext blob (content) into a rendered hast
 *
 * note that this function is latent and CPU-bound, particularly if it is the first invokation as it will await getHighlighter (can take up to a second)
 */
export async function renderHast(
  content: string,
  lang: BundledLanguage | undefined,
  theme: "vitesse" | "gitdot",
  transformers: ShikiTransformer[] = [],
) {
  const highlighter = await getHighlighter(lang);
  if (theme === "vitesse") {
    return highlighter.codeToHast(content, {
      lang: lang ?? "plaintext",
      themes: { light: "vitesse-light", dark: "vitesse-dark" },
      defaultColor: "light",
      transformers,
    });
  }

  return highlighter.codeToHast(content, {
    lang: lang ?? "plaintext",
    theme: "gitdot-light",
    transformers,
  });
}

/**
 * turns a plaintext blob (content) into a list of rendered spans
 *
 * this is meant to be used in diffing views and requires what the set of changed lines and what side the diff is on (e.g., show red or green)
 */
export async function renderSpans(
  content: string,
  lang: BundledLanguage | undefined,
  theme: "vitesse" | "gitdot",
  side: "left" | "right",
  changedLines: Set<number>,
): Promise<Element[]> {
  const hast = await renderHast(content, lang, theme, [
    {
      pre(node) {
        this.addClassToHast(node, "outline-none");
      },
      code(node) {
        this.addClassToHast(node, "flex flex-col");
      },
      line(node, lineNumber) {
        node.type = "element";
        node.tagName = "diffline";
        node.properties["data-line-number"] = lineNumber;

        if (changedLines.has(lineNumber - 1)) {
          node.properties["data-line-type"] =
            side === "left" ? "removed" : "added";
        } else {
          node.properties["data-line-type"] = "normal";
        }
      },
    },
  ]);

  const root = hast as Root;
  const pre = root.children[0] as Element;
  const code = pre.children[0] as Element;

  const lines = code.children.filter(
    (child): child is Element => child.type === "element",
  );
  for (const line of lines) {
    decorateLineNode(line);
  }
  return lines;
}

function decorateLineNode(lineNode: Element): void {
  const newChildren: ElementContent[] = [];
  let charOffset = 0;

  for (const child of lineNode.children) {
    if (child.type !== "element") {
      newChildren.push(child);
      continue;
    }

    const spanChildren = [...child.children];

    let leadingSpaces = "";
    const firstChild = spanChildren[0];
    if (firstChild?.type === "text") {
      const [spaces, rest] = takeLeadingSpaces(firstChild.value);
      if (spaces) {
        leadingSpaces = spaces;
        if (rest) spanChildren[0] = { type: "text", value: rest };
        else spanChildren.shift();
      }
    }

    let trailingSpaces = "";
    const lastChild = spanChildren[spanChildren.length - 1];
    if (lastChild?.type === "text") {
      const [rest, spaces] = takeTrailingSpaces(lastChild.value);
      if (spaces) {
        trailingSpaces = spaces;
        if (rest)
          spanChildren[spanChildren.length - 1] = { type: "text", value: rest };
        else spanChildren.pop();
      }
    }

    const contentLength = spanChildren.reduce(
      (sum, c) => sum + getTextLength(c),
      0,
    );

    const makeToken = (
      props: Record<string, unknown>,
      c: ElementContent[],
    ): Element => ({
      ...child,
      properties: {
        ...child.properties,
        ...(child.properties.class
          ? { class: [...(child.properties.class as string[]), "diff-token"] }
          : { class: ["diff-token"] }),
        ...props,
      },
      children: c,
    });

    if (leadingSpaces) {
      newChildren.push(
        makeToken(
          {
            "data-char-start": charOffset,
            "data-char-end": charOffset + leadingSpaces.length,
          },
          [{ type: "text", value: leadingSpaces }],
        ),
      );
      charOffset += leadingSpaces.length;
    }
    if (spanChildren.length > 0) {
      newChildren.push(
        makeToken(
          {
            "data-char-start": charOffset,
            "data-char-end": charOffset + contentLength,
          },
          spanChildren,
        ),
      );
      charOffset += contentLength;
    }
    if (trailingSpaces) {
      newChildren.push(
        makeToken(
          {
            "data-char-start": charOffset,
            "data-char-end": charOffset + trailingSpaces.length,
          },
          [{ type: "text", value: trailingSpaces }],
        ),
      );
      charOffset += trailingSpaces.length;
    }
  }

  lineNode.children = newChildren;
}

function getTextLength(node: ElementContent): number {
  if (node.type === "text") return node.value.length;
  if (node.type === "element") {
    return node.children.reduce((sum, c) => sum + getTextLength(c), 0);
  }
  return 0;
}

function takeLeadingSpaces(value: string): [spaces: string, rest: string] {
  let i = 0;
  while (i < value.length && (value[i] === " " || value[i] === "\t")) i++;
  return [value.slice(0, i), value.slice(i)];
}

function takeTrailingSpaces(value: string): [rest: string, spaces: string] {
  let i = value.length;
  while (i > 0 && (value[i - 1] === " " || value[i - 1] === "\t")) i--;
  return [value.slice(0, i), value.slice(i)];
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
