"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { useParams } from "next/navigation";
import { useEffect, useRef, useState } from "react";
import { MarkdownBody } from "@/(main)/[owner]/[repo]/ui/markdown/markdown-body";
import { DatabaseProvider } from "@/provider/database";
import Link from "@/ui/link";
import { getFolderEntries } from "../../util";
import { FileBody } from "./file-body";
import { FolderToc, type TocHeader } from "./folder-toc";

function slugify(text: string): string {
  return text
    .toLowerCase()
    .replace(/[^\w\s-]/g, "")
    .replace(/[\s_]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

function extractHeaders(markdown: string): TocHeader[] {
  const headerRegex = /^(#{1,5})\s+(.+)$/gm;
  const headers: TocHeader[] = [];
  let first = true;
  for (const match of markdown.matchAll(headerRegex)) {
    if (first) {
      first = false;
      continue;
    }
    const text = match[2].replace(/[*_`[\]]/g, "").trim();
    headers.push({ level: match[1].length, text, slug: slugify(text) });
  }
  return headers;
}

type TreeLine = {
  prefix: string;
  connector: string;
  name: string;
  path: string;
  isTree: boolean;
  isExpanded: boolean;
  depth: number;
};

function flattenTree(
  folderPath: string,
  paths: RepositoryPathsResource,
  expandedPaths: Set<string>,
  prefix = "",
  depth = 0,
): TreeLine[] {
  const entries = getFolderEntries(folderPath, paths);
  const lines: TreeLine[] = [];
  for (let i = 0; i < entries.length; i++) {
    const entry = entries[i];
    const name = entry.path.split("/").pop()!;
    const isTree = entry.path_type === "tree";
    const isLast = i === entries.length - 1;
    const connector = depth === 0 ? "" : (isLast ? "└─ " : "├─ ");
    const isExpanded = isTree && expandedPaths.has(entry.path);
    lines.push({ prefix, connector, name, path: entry.path, isTree, isExpanded, depth });
    if (isExpanded) {
      const childPrefix = depth === 0 ? " " : prefix + (isLast ? "  " : "│ ");
      lines.push(...flattenTree(entry.path, paths, expandedPaths, childPrefix, depth + 1));
    }
  }
  return lines;
}

function flattenAll(
  folderPath: string,
  paths: RepositoryPathsResource,
  prefix = "",
  depth = 0,
): Omit<TreeLine, "isExpanded">[] {
  const entries = getFolderEntries(folderPath, paths);
  const lines: Omit<TreeLine, "isExpanded">[] = [];
  for (let i = 0; i < entries.length; i++) {
    const entry = entries[i];
    const name = entry.path.split("/").pop()!;
    const isTree = entry.path_type === "tree";
    const isLast = i === entries.length - 1;
    const connector = depth === 0 ? "" : (isLast ? "└─ " : "├─ ");
    lines.push({ prefix, connector, name, path: entry.path, isTree, depth });
    if (isTree) {
      const childPrefix = depth === 0 ? " " : prefix + (isLast ? "  " : "│ ");
      lines.push(...flattenAll(entry.path, paths, childPrefix, depth + 1));
    }
  }
  return lines;
}

type Preview = { kind: "file"; hast: Root } | { kind: "folder"; lines: Omit<TreeLine, "isExpanded">[] };

export function FolderViewer({
  folderPath,
  readme,
}: {
  folderPath?: string;
  readme?: string | null;
}) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const [paths, setPaths] = useState<RepositoryPathsResource | null>(null);
  const [expandedPaths, setExpandedPaths] = useState<Set<string>>(new Set());
  const [preview, setPreview] = useState<Preview | null>(null);
  const dbRef = useRef<DatabaseProvider | null>(null);
  const pathsRef = useRef<RepositoryPathsResource | null>(null);

  useEffect(() => {
    const db = new DatabaseProvider(owner, repo);
    dbRef.current = db;
    db.getPaths().then((p) => {
      if (!p) return;
      setPaths(p);
      pathsRef.current = p;
      const topLevel = getFolderEntries(folderPath ?? "", p);
      setExpandedPaths(new Set(topLevel.filter((e) => e.path_type === "tree").map((e) => e.path)));
    });
  }, [owner, repo]);

  const toggleFolder = (path: string) => {
    setExpandedPaths((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });
  };

  const handleHover = (path: string, isTree: boolean) => {
    if (isTree) {
      if (pathsRef.current) {
        setPreview({ kind: "folder", lines: flattenAll(path, pathsRef.current) });
      }
      return;
    }
    dbRef.current?.getHast(path).then((hast) => {
      if (hast) setPreview({ kind: "file", hast });
      else setPreview(null);
    });
  };

  if (readme) {
    return (
      <div className="flex w-full h-full min-h-0 overflow-hidden">
        <div
          data-page-scroll
          className="flex flex-col flex-1 min-w-0 overflow-auto scrollbar-thin"
        >
          <div className="border-b px-8 py-6 max-w-4xl mx-auto w-full">
            <MarkdownBody content={readme} />
          </div>
        </div>
        <FolderToc headers={extractHeaders(readme)} />
      </div>
    );
  }

  if (!paths) return null;

  const lines = flattenTree(folderPath ?? "", paths, expandedPaths);

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div
        data-page-scroll
        className="flex flex-col w-1/2 shrink-0 border-r overflow-auto scrollbar-thin px-4 py-2"
        onMouseLeave={() => setPreview(null)}
      >
        {lines.map((line) =>
          line.isTree ? (
            <button
              key={line.path}
              type="button"
              className="flex font-mono text-sm leading-6 px-1 rounded hover:bg-accent cursor-pointer text-left w-full"
              onMouseEnter={() => handleHover(line.path, true)}
              onClick={() => toggleFolder(line.path)}
            >
              <span className="text-muted-foreground whitespace-pre select-none">{line.prefix}{line.connector}</span>
              <span>{line.name}/</span>
            </button>
          ) : (
            <Link
              key={line.path}
              href={`/${owner}/${repo}/${line.path}`}
              className="flex font-mono text-sm leading-6 px-1 rounded hover:bg-accent cursor-pointer"
              onMouseEnter={() => handleHover(line.path, false)}
            >
              <span className="text-muted-foreground whitespace-pre select-none">{line.prefix}{line.connector}</span>
              <span>{line.name}</span>
            </Link>
          )
        )}
      </div>
      <div className="flex-1 min-w-0 overflow-auto scrollbar-thin px-4 py-2">
        {preview?.kind === "file" && (
          <FileBody selectedLines={null} hast={preview.hast} />
        )}
        {preview?.kind === "folder" && (
          <div className="flex flex-col font-mono text-sm">
            {preview.lines.map((line) => (
              <div key={line.path} className="flex leading-6 px-1" style={{ paddingLeft: `${line.depth * 1}rem` }}>
                <span className="text-muted-foreground whitespace-pre select-none">{line.connector}</span>
                <span className={line.isTree ? "" : "text-muted-foreground"}>{line.isTree ? `${line.path}/` : line.path}</span>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
