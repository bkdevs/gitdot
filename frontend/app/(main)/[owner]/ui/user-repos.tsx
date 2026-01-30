import type { UserRepositoryResponse } from "@/lib/dto";
import Link from "@/ui/link";
import { formatDate } from "@/util";
import { CreateRepoButton } from "./create-repo-button";

export function UserRepos({
  user,
  repos,
}: {
  user: string;
  repos: UserRepositoryResponse[];
}) {
  return (
    <div className="flex flex-col w-full">
      <RepoHeader />
      {repos.map((repo) => (
        <Link
          className="flex flex-row items-center px-2 py-2 border-b hover:bg-accent/50 select-none"
          key={repo.id}
          href={`/${user}/${repo.name}`}
        >
          <div className="flex flex-col">
            <div className="flex flex-row text-sm">{repo.name}</div>
            <div className="flex flex-row text-xs text-muted-foreground pt-0.5">
              {repo.visibility === "public" ? "Public" : "Private"} •{" "}
              {formatDate(new Date(repo.created_at))}
            </div>
          </div>
        </Link>
      ))}
    </div>
  );
}

function RepoHeader() {
  return (
    <div className="flex items-center justify-between border-b pl-2 h-9">
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Repos
      </h3>
      <CreateRepoButton />
    </div>
  );
}
