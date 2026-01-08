import type { BundledLanguage } from "shiki";
import type { RepositoryTree } from "@/lib/dto";

export type LineSelection = {
  start: number;
  end: number;
};

export function parseLineSelection(
  param: string | string[] | undefined,
): LineSelection | null {
  if (!param || typeof param !== "string") return null;
  if (param.includes("-")) {
    const [start, end] = param.split("-").map(Number);
    if (
      !Number.isNaN(start) &&
      !Number.isNaN(end) &&
      start > 0 &&
      end >= start
    ) {
      return { start, end };
    }
  } else {
    const line = Number(param);
    if (!Number.isNaN(line) && line > 0) {
      return { start: line, end: line };
    }
  }
  return null;
}

export function formatLineSelection(selection: LineSelection): string {
  return selection.start === selection.end
    ? `${selection.start}`
    : `${selection.start}-${selection.end}`;
}

export function inferLanguage(filePath: string): BundledLanguage | null {
  console.log(filePath);
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

  return extension && extensionMap[extension] ? extensionMap[extension] : null;
}

export function parseRepositoryTree(tree: RepositoryTree): {
  filePaths: Set<string>;
  folders: Map<string, string[]>;
} {
  const filePaths = new Set<string>();
  const folders = new Map<string, string[]>();

  for (const entry of tree.entries) {
    filePaths.add(entry.path);

    const segments = entry.path.split("/");
    const fileName = segments[segments.length - 1];

    if (segments.length === 1) {
      // Root-level entry - add to root folder ("")
      if (!folders.has("")) {
        folders.set("", []);
      }
      folders.get("")?.push(fileName);
    } else if (segments.length > 1) {
      // Nested entry - add to parent folder
      const folder = segments.slice(0, -1).join("/");
      if (!folders.has(folder)) {
        folders.set(folder, []);
      }
      folders.get(folder)?.push(fileName);
    }
  }

  for (const arr of folders.values()) {
    arr.sort();
  }
  return { filePaths, folders };
}

export type FolderFile = {
  path: string;
  type: "file" | "folder";
};

export const getFolderFiles = (
  folderPath: string,
  folders: Map<string, string[]>,
): FolderFile[] => {
  const files = folders.get(folderPath);
  return (
    files?.map((filePath) => ({
      path: filePath,
      type: folders.has(`${folderPath}/${filePath}`) ? "folder" : "file",
    })) || []
  );
};

export function getParentPath(currentPath: string): string {
  if (!currentPath) {
    return "";
  }

  const segments = currentPath.split("/");
  if (segments.length === 1) {
    return "";
  } else {
    return segments.slice(0, -1).join("/");
  }
}
