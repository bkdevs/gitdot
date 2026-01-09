import { File, Folder } from "lucide-react";
import Link from "next/link";
import type { RepositoryTreeEntry } from "@/lib/dto";
import { timeAgo } from "@/util";
import { FolderHeader } from "./folder-header";

export async function FolderViewer({
  repo,
  folderPath,
  folderEntries,
}: {
  repo: string;
  folderPath: string;
  folderEntries: RepositoryTreeEntry[];
}) {
  return (
    <div className="flex flex-col w-full h-screen">
      <FolderHeader repo={repo} folderPath={folderPath} />
      <div className="flex-1 overflow-hidden flex flex-col">
        {folderEntries.map((entry) => (
          <FolderEntryRow
            key={entry.path}
            entry={entry}
            href={`/${repo}/${entry.path}`}
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
  entry: RepositoryTreeEntry;
  href: string;
}) {
  return (
    <Link
      className="flex flex-row w-full px-2 h-9 items-center border-b hover:bg-accent/50 select-none cursor-default text-sm"
      href={href}
      prefetch={true}
    >
      {entry.entry_type === "blob" ? (
        <File className="size-4" />
      ) : (
        <Folder className="size-4" />
      )}
      <span className="ml-2">{entry.path.split("/").pop()}</span>
      <span className="ml-auto w-96 truncate">{entry.commit.message}</span>
      <span className="text-primary/60 ml-2">
        {timeAgo(new Date(entry.commit.date))}
      </span>
    </Link>
  );
}
