import type { RepositoryPathResource } from "gitdot-api";
import { File, Folder } from "lucide-react";
import Link from "@/ui/link";

export function PathsFolderViewer({
  owner,
  repo,
  entries,
}: {
  owner: string;
  repo: string;
  entries: RepositoryPathResource[];
}) {
  return (
    <div className="flex flex-col w-full flex-1">
      <div className="flex-1 overflow-hidden flex flex-col">
        {entries.map((entry) => (
          <Link
            key={entry.path}
            className="flex w-full pl-2 h-9 items-center border-b hover:bg-accent/50 select-none cursor-default text-sm"
            href={`/${owner}/${repo}/${entry.path}`}
            prefetch={true}
          >
            <span className="flex items-center min-w-0">
              {entry.path_type === "blob" ? (
                <File className="size-4 shrink-0" />
              ) : (
                <Folder className="size-4 shrink-0" />
              )}
              <span className="ml-2 truncate font-mono">
                {entry.path.split("/").pop()}
              </span>
            </span>
          </Link>
        ))}
      </div>
    </div>
  );
}
