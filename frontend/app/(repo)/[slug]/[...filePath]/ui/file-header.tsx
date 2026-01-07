import { RepositoryCommit, RepositoryFile } from "@/lib/dto";

export function FileHeader({ file, commit }: { file: RepositoryFile, commit: RepositoryCommit }) {
  return (
    <div className="flex flex-row w-full h-9 items-center justify-between border-b px-2 text-sm font-mono">
      <span>{file.path}</span>
      <span>
        <span>{commit.author} • {commit.message}, {commit.date}</span>
      </span>
    </div>
  );
}
