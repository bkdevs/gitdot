"use client";

import type { RepositoryPathResource } from "gitdot-api";
import { ChevronDown, ChevronRight, File, Folder } from "lucide-react";
import { useState } from "react";
import { MarkdownBody } from "@/(main)/[owner]/[repo]/ui/markdown/markdown-body";
import Link from "@/ui/link";
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

export function FolderViewer({
  owner,
  repo,
  folderPath,
  entries,
  readme,
  allFiles,
}: {
  owner: string;
  repo: string;
  folderPath?: string;
  entries: RepositoryPathResource[];
  readme?: string | null;
  allFiles?: RepositoryPathResource[] | null;
}) {
  const sortedEntries = entries.toSorted((a, b) => {
    if (a.path_type === b.path_type) {
      return a.path.localeCompare(b.path);
    }
    return a.path_type === "tree" ? -1 : 1;
  });

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

  if (allFiles && allFiles.length > 0) {
    return (
      <RecursiveFolderView
        owner={owner}
        repo={repo}
        folderPath={folderPath ?? ""}
        allFiles={allFiles}
      />
    );
  }

  return (
    <div
      data-page-scroll
      className="flex flex-col flex-1 w-full min-w-0 overflow-auto scrollbar-thin"
    >
      {sortedEntries.map((entry) => (
        <FolderEntryRow
          key={entry.path}
          entry={entry}
          href={`/${owner}/${repo}/${entry.path}`}
        />
      ))}
    </div>
  );
}

function RecursiveFolderView({
  owner,
  repo,
  folderPath,
  allFiles,
}: {
  owner: string;
  repo: string;
  folderPath: string;
  allFiles: RepositoryPathResource[];
}) {
  const [collapsed, setCollapsed] = useState<Set<string>>(
    () => new Set(allFiles.filter((e) => e.path_type === "tree").map((e) => e.path)),
  );

  const prefix = folderPath ? `${folderPath}/` : "";
  const sorted = allFiles.toSorted((a, b) => a.path.localeCompare(b.path));

  const visible = sorted.filter((entry) => {
    const relative = entry.path.slice(prefix.length);
    const parts = relative.split("/");
    for (let i = 1; i < parts.length; i++) {
      const ancestorPath = prefix + parts.slice(0, i).join("/");
      if (collapsed.has(ancestorPath)) return false;
    }
    return true;
  });

  const toggle = (path: string) => {
    setCollapsed((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });
  };

  return (
    <div
      data-page-scroll
      className="flex flex-col flex-1 w-full min-w-0 overflow-auto scrollbar-thin"
    >
      {visible.map((entry) => {
        const relative = entry.path.slice(prefix.length);
        const depth = relative.split("/").length - 1;
        const name = relative.split("/").pop() ?? relative;
        const isFolder = entry.path_type === "tree";
        const isCollapsed = collapsed.has(entry.path);

        if (isFolder) {
          return (
            <button
              key={entry.path}
              type="button"
              style={{ paddingLeft: `${0.5 + depth * 1}rem` }}
              className="flex w-full h-9 items-center border-b hover:bg-accent/50 select-none cursor-default text-sm text-left"
              onClick={() => toggle(entry.path)}
            >
              {isCollapsed ? (
                <ChevronRight className="size-3 shrink-0 text-muted-foreground" />
              ) : (
                <ChevronDown className="size-3 shrink-0 text-muted-foreground" />
              )}
              <Folder className="size-4 shrink-0 ml-1" />
              <span className="ml-2 truncate font-mono">{name}</span>
            </button>
          );
        }

        return (
          <Link
            key={entry.path}
            data-page-item
            tabIndex={-1}
            style={{ paddingLeft: `${0.5 + depth * 1}rem` }}
            className="flex w-full h-9 items-center border-b focus:bg-accent/50 select-none cursor-default text-sm focus:outline-none"
            href={`/${owner}/${repo}/${entry.path}`}
            prefetch={true}
          >
            <File className="size-4 shrink-0 ml-4" />
            <span className="ml-2 truncate font-mono">{name}</span>
          </Link>
        );
      })}
    </div>
  );
}

function FolderEntryRow({
  entry,
  href,
  label,
  depth = 0,
}: {
  entry: RepositoryPathResource;
  href: string;
  label?: string;
  depth?: number;
}) {
  return (
    <Link
      data-page-item
      tabIndex={-1}
      style={{ paddingLeft: `${0.5 + depth * 1}rem` }}
      className="grid grid-cols-[1fr_300px_150px] w-full h-9 items-center border-b focus:bg-accent/50 select-none cursor-default text-sm focus:outline-none"
      href={href}
      prefetch={true}
    >
      <span className="flex items-center min-w-0">
        {entry.path_type === "blob" ? (
          <File className="size-4 shrink-0" />
        ) : (
          <Folder className="size-4 shrink-0" />
        )}
        <span className="ml-2 truncate font-mono">
          {label ?? entry.path.split("/").pop()}
        </span>
      </span>
      <span className="truncate" />
    </Link>
  );
}
