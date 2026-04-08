import { listUserRepositories } from "@/dal";
import Link from "@/ui/link";

export async function UserRepos({ owner }: { owner: string }) {
  const repos = await listUserRepositories(owner);

  if (!repos?.length) return null;

  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold text-sm">repos</p>
      <div className="flex flex-col items-end gap-1">
        {repos.map((repo) => (
          <Link key={repo.id} href={`/${owner}/${repo.name}`}>
            <span className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200">
              {repo.name}
            </span>
          </Link>
        ))}
      </div>
    </div>
  );
}
