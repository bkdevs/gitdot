import type { RepositoryFile, RepositoryTree, RepositoryTreeEntry } from "@/lib/dto";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import type { BundledLanguage } from "shiki";
import { codeToHast } from "shiki";
import { FileLine } from "./[...filePath]/ui/file-line";

export function fuzzyMatch(query: string, target: string): boolean {
  const lowerQuery = query.toLowerCase();
  const lowerTarget = target.toLowerCase();
  let queryIndex = 0;

  for (
    let i = 0;
    i < lowerTarget.length && queryIndex < lowerQuery.length;
    i++
  ) {
    if (lowerTarget[i] === lowerQuery[queryIndex]) {
      queryIndex++;
    }
  }

  return queryIndex === lowerQuery.length;
}

export function getMockPreview(entry: RepositoryTreeEntry): string {
  if (entry.entry_type === "tree") {
    return "// Directory\n// Contents preview will be available soon";
  }

  const ext = entry.path.split(".").pop() || "";

  if (ext === "yaml" || ext === "yml") {
    return `# \n\npackages:\n  - example\n\ndependencies:\n  - typescript: ^5.0.0\n  - react: ^19.0.0`;
  }

  if (ext === "json") {
    return `{\n  "name": "",\n  "version": "1.0.0",\n  "description": "Mock preview"\n}`;
  }

  if (ext === "ts" || ext === "tsx" || ext === "js" || ext === "jsx") {
    return `// \n\nexport default function Component() {\n  return <div>Preview coming soon</div>;\n}`;
  }

  if (ext === "md") {
    return `# \n\nThis is a mock preview of the file content.\n\nActual content will be loaded soon.`;
  }

  return `// \n// File preview will be a}vailable soon\n// Type: ${entry.entry_type}\n// SHA: ${entry.sha}`;
}

export function inferLanguage(filePath: string): BundledLanguage | null {
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

export async function fileToJsx(content: string, path: string): Promise<JSX.Element> {
}

export function parseRepositoryTree(tree: RepositoryTree): {
  entries: Map<string, RepositoryTreeEntry>;
  folders: Map<string, string[]>;
} {
  const entries = new Map<string, RepositoryTreeEntry>();
  const folders = new Map<string, string[]>();

  for (const entry of tree.entries) {
    entries.set(entry.path, entry);

    const segments = entry.path.split("/");
    const fileName = segments[segments.length - 1];

    if (segments.length === 1) {
      if (!folders.has("")) {
        folders.set("", []);
      }
      folders.get("")?.push(fileName);
    } else if (segments.length > 1) {
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
  return { entries, folders };
}

export type FolderFile = {
  path: string;
  type: "file" | "folder";
};

export const getFolderEntries = (
  folderPath: string,
  folders: Map<string, string[]>,
  entries: Map<string, RepositoryTreeEntry>,
): RepositoryTreeEntry[] => {
  const files = folders.get(folderPath);
  if (!files) return [];
  return files
    .map((fileName) =>
      entries.get(folderPath ? `${folderPath}/${fileName}` : fileName),
    )
    .filter((entry): entry is RepositoryTreeEntry => entry !== undefined);
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
