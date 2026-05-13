import type { RepositoryResource } from "gitdot-api";
import Link from "@/ui/link";

export function UserStars({ stars }: { stars: RepositoryResource[] }) {
  if (!stars.length) return null;

  return (
    <div className="flex flex-col items-start w-full min-w-0">
      <p className="font-semibold text-sm mb-0.5">stars</p>
      {stars.map((repo) => (
        <Link
          key={repo.id}
          href={`/${repo.owner}/${repo.name}`}
          className="block w-full truncate text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200"
        >
          <span className="text-muted-foreground">{repo.owner}/</span>
          {repo.name}
        </Link>
      ))}
    </div>
  );
}
