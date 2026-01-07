import Link from "next/link";
import type { RepositoryCommit, RepositoryFile } from "@/lib/dto";
import { timeAgo } from "@/util";

export function FileHeader({
  repo,
  file,
  commit,
}: {
  repo: string;
  file: RepositoryFile;
  commit: RepositoryCommit;
}) {
  var path = "";
  const pathSegments = file.path.split("/");
  const pathLinks = pathSegments.map((segment, index) => {
    path += `/${segment}`;
    return (
      <Link className="hover:underline" href={`/${repo}${path}`} key={segment}>
        {segment}
        {index !== pathSegments.length - 1 && "/"}
      </Link>
    );
  });

  return (
    <div className="flex flex-row w-full h-9 items-center justify-between border-b px-2 text-sm font-mono">
      <div>{pathLinks}</div>
      <div>
        {commit.author} • {commit.message}
        <span className="text-muted-foreground">
          {timeAgo(new Date(commit.date))}
        </span>
      </div>
    </div>
  );
}
