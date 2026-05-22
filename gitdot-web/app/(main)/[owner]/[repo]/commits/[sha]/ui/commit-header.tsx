import type { RepositoryCommitResource } from "gitdot-api";
import { UserImage } from "@/(main)/[owner]/ui/user/user-image";
import Link from "@/ui/link";
import { formatDateTime } from "@/util";

export function CommitHeader({
  commit,
  owner,
  repo,
}: {
  commit: RepositoryCommitResource;
  owner: string;
  repo: string;
}) {
  const date = new Date(commit.date);

  return (
    <div className="flex flex-col">
      <div className="flex items-center gap-2 text-sm text-muted-foreground">
        <UserImage userId={commit.author.id} px={20} />
        <span>
          <Link
            href={`/${commit.author.name}`}
            className="underline hover:text-foreground transition-colors duration-200"
            prefetch={true}
          >
            {commit.author.name}
          </Link>
          {" in "}
          <span className="font-mono">
            <Link
              href={`/${owner}`}
              className="underline decoration-transparent hover:decoration-current transition-colors duration-200"
              prefetch={true}
            >
              {owner}
            </Link>
            /
            <Link
              href={`/${owner}/${repo}`}
              className="font-medium text-foreground underline decoration-transparent hover:decoration-current transition-colors duration-200"
              prefetch={true}
            >
              {repo}
            </Link>
          </span>
        </span>
      </div>
      <div className="text-sm text-foreground whitespace-pre-wrap mt-1">
        {commit.message}
      </div>
      <div className="flex items-baseline gap-1 text-xs font-mono text-muted-foreground mt-1">
        <span>{formatDateTime(date)}</span>
        <span>·</span>
        <span>{commit.sha.substring(0, 7)}</span>
      </div>
    </div>
  );
}
