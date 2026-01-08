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
  const parentFolder = folderPath.split("/").slice(0, -1).join("/") || "/";
  return (
    <div className="flex flex-col w-full h-screen">
      <FolderHeader repo={repo} folderPath={folderPath} />
      <div className="flex-1 overflow-hidden flex flex-col">
        <FolderFileRow
          file={{ path: "..", type: "folder" }}
          href={`/${repo}/${parentFolder}`}
        />
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

function FolderFileRow({ file, href }: { file: FolderFile; href: string }) {
  return (
    <Link
      className="flex flex-row w-full px-2 h-9 items-center border-b hover:bg-accent/50 select-none cursor-default"
      href={href}
    >
      {file.type === "file" ? (
        <File className="size-4" />
      ) : (
        <Folder className="size-4" />
      )}
      <span className="ml-2 text-sm font-mono">{file.path}</span>
    </Link>
  );
}
