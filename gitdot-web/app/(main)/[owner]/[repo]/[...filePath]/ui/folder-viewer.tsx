import type { RepositoryPathResource } from "gitdot-api";
import { File, Folder } from "lucide-react";
import { MarkdownBody } from "@/(main)/[owner]/[repo]/ui/markdown/markdown-body";
import Link from "@/ui/link";

export function FolderViewer({
  owner,
  repo,
  entries,
  readme,
}: {
  owner: string;
  repo: string;
  entries: RepositoryPathResource[];
  readme?: string | null;
}) {
  const sortedEntries = entries.toSorted((a, b) => {
    if (a.path_type === b.path_type) {
      return a.path.localeCompare(b.path);
    }
    return a.path_type === "tree" ? -1 : 1;
  });

  return (
    <div className="flex flex-col w-full flex-1 overflow-auto">
      {readme && (
        <div className="border-b px-4 py-3">
          <MarkdownBody content={readme} />
        </div>
      )}
      <div className="flex-1 overflow-hidden flex flex-col">
        {sortedEntries.map((entry) => (
          <FolderEntryRow
            key={entry.path}
            entry={entry}
            href={`/${owner}/${repo}/${entry.path}`}
          />
        ))}
      </div>
    </div>
  );
}

function FolderEntryRow({
  entry,
  href,
}: {
  entry: RepositoryPathResource;
  href: string;
}) {
  return (
    <Link
      data-page-item
      tabIndex={-1}
      className="grid grid-cols-[1fr_300px_150px] w-full pl-2 h-9 items-center border-b focus:bg-accent/50 select-none cursor-default text-sm focus:outline-none"
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
          {entry.path.split("/").pop()}
        </span>
      </span>
      <span className="truncate" />
    </Link>
  );
}
