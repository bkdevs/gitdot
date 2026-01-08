import { File, Folder } from "lucide-react";
import Link from "next/link";
import type { FolderFile } from "../util";
import { FolderHeader } from "./folder-header";

export async function FolderViewer({
  repo,
  folderPath,
  folderFiles,
}: {
  repo: string;
  folderPath: string;
  folderFiles: FolderFile[];
}) {
  return (
    <div className="flex flex-col w-full h-screen">
      <FolderHeader folderPath={folderPath} />
      <div className="flex-1 overflow-hidden flex flex-col">
        {folderFiles.map((file) => (
          <FolderFileRow
            key={file.path}
            file={file}
            href={`/${repo}/${folderPath}/${file.path}`}
          />
        ))}
      </div>
    </div>
  );
}

function FolderFileRow({
  file,
  href,
  showCommit = true,
}: {
  file: FolderFile;
  href: string;
  showCommit?: boolean;
}) {
  return (
    <Link
      className="flex flex-row w-full px-2 h-9 items-center border-b hover:bg-accent/50 select-none cursor-default text-sm"
      href={href}
      prefetch={true}
    >
      {file.type === "file" ? (
        <File className="size-4" />
      ) : (
        <Folder className="size-4" />
      )}
      <span className="ml-2">{file.path}</span>
      {showCommit && (
        <>
          <span className="ml-auto w-96 truncate">
            add user authentication with JWT token support
          </span>
          <span className="text-primary/60 ml-2">4 months ago</span>
        </>
      )}
    </Link>
  );
}
