"use client";

import Link from "@/ui/link";
import type { RepositoryPathsResource } from "gitdot-api";
export type { TreeRowData } from "./folder-tree-row";

export function FolderTreeHeader({
  path,
  paths,
  owner,
  repo,
}: {
  path: string;
  paths: RepositoryPathsResource;
  owner: string;
  repo: string;
}) {
  const prefix = path ? `${path}/` : "";
  const under = paths.entries.filter(
    (e) => e.path.startsWith(prefix) && e.path !== path,
  );
  const fileCount = under.filter((e) => e.path_type === "blob").length;

  return (
    <div className="font-mono text-sm h-6 flex items-center justify-between pl-3 pr-2 pt-2 shrink-0">
      <div className="flex items-center">
        <Link href={`/${owner}`} className="hover:underline">
          {owner}
        </Link>
        <span>/</span>
        <Link href={`/${owner}/${repo}/files`} className="hover:underline">
          {repo}
        </Link>
        {path?.length > 0 &&
          path?.split("/").map((seg, i, arr) => (
            // biome-ignore lint/suspicious/noArrayIndexKey: stable path segments
            <span key={i}>
              <span>/</span>
              <Link
                href={`/${owner}/${repo}/${arr.slice(0, i + 1).join("/")}`}
                className="hover:underline"
              >
                {seg}
              </Link>
            </span>
          ))}
        <span>/</span>
      </div>
      <span className="text-xs text-muted-foreground">{fileCount} files</span>
    </div>
  );
}
