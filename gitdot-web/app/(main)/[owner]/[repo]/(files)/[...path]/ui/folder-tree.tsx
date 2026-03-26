"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import { useRef, useState } from "react";
import { expandPaths, buildTreeRows } from "../util";
import { FolderShortcuts } from "./folder-shortcuts";
import { FolderTreeHeader } from "./folder-tree-header";
import { FolderTreeRow } from "./folder-tree-row";
export type { TreeRowData } from "./folder-tree-row";

export function FolderTree({
  owner,
  repo,
  path,
  paths,
  previewPath,
  setPreviewPath,
  initialExpanded,
  activePath,
}: {
  owner: string;
  repo: string;
  path: string;
  paths: RepositoryPathsResource;
  previewPath?: string | null;
  setPreviewPath: (path: string) => void;
  initialExpanded?: Set<string>;
  activePath?: string;
}) {
  const [expandedPaths, setExpandedPaths] = useState<Set<string>>(() =>
    initialExpanded ?? expandPaths(path, paths, 1),
  );

  const mouseMoved = useRef(false);

  const toggleFolder = (folderPath: string) => {
    setExpandedPaths((prev) => {
      const next = new Set(prev);
      if (next.has(folderPath)) {
        for (const p of next) {
          if (p === folderPath || p.startsWith(`${folderPath}/`))
            next.delete(p);
        }
      } else {
        next.add(folderPath);
      }
      return next;
    });
  };

  const rows = buildTreeRows(path ?? "", paths, expandedPaths);

  return (
    <div
      data-page-scroll
      className="flex flex-col h-full overflow-y-auto scrollbar-thin"
      onMouseMove={() => {
        mouseMoved.current = true;
      }}
    >
      <FolderShortcuts
      rows={rows}
      hoveredPath={previewPath ?? null}
      onHover={(p) => {
        mouseMoved.current = false;
        setPreviewPath(p);
      }}
      onToggle={toggleFolder}
      />
      <FolderTreeHeader path={path} paths={paths} owner={owner} repo={repo} />
      {rows.map((row) => (
        <FolderTreeRow
          key={row.path}
          row={row}
          owner={owner}
          repo={repo}
          absolutePaths={false}
          focused={previewPath === row.path}
          active={activePath === row.path}
          onMouseEnter={() => {
            if (!mouseMoved.current) return;
            setPreviewPath(row.path);
          }}
          onClick={toggleFolder}
        />
      ))}
    </div>
  );
}
