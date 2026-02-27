import type { RepositoryTreeEntryResource } from "gitdot-api-ts";
import { File, Folder } from "lucide-react";
import Link from "@/ui/link";
import { timeAgo } from "@/util";

export async function FolderViewer({
  owner,
  repo,
  folderEntries,
}: {
  owner: string;
  repo: string;
  folderEntries: RepositoryTreeEntryResource[];
}) {
  const sortedEntries = folderEntries.toSorted((a, b) => {
    if (a.entry_type === b.entry_type) {
      return a.path.localeCompare(b.path);
    }
    return a.entry_type === "tree" ? -1 : 1;
  });

  return (
    <div className="flex flex-col w-full flex-1">
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
  entry: RepositoryTreeEntryResource;
  href: string;
}) {
  const author = entry.commit.author.name;

  return (
    <Link
      className="grid grid-cols-[1fr_300px_150px] w-full pl-2 h-9 items-center border-b hover:bg-accent/50 select-none cursor-default text-sm"
      href={href}
      prefetch={true}
    >
      <span className="flex items-center min-w-0">
        {entry.entry_type === "blob" ? (
          <File className="size-4 shrink-0" />
        ) : (
          <Folder className="size-4 shrink-0" />
        )}
        <span className="ml-2 truncate font-mono">
          {entry.path.split("/").pop()}
        </span>
      </span>
      <span className="truncate">{entry.commit.message}</span>
      <span className="text-primary/60 ml-4 whitespace-nowrap">
        {author} • {timeAgo(new Date(entry.commit.date))}
      </span>
    </Link>
  );
}
