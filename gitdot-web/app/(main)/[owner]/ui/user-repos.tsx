import { listUserRepositories } from "@/dal";
import Link from "@/ui/link";

export async function UserRepos({ owner }: { owner: string }) {
  const repos = await listUserRepositories(owner);

  if (!repos?.length) return null;

  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold text-sm mb-0.5">repos</p>
      {repos.map((repo) => (
        <Link
          key={repo.id}
          href={`/${owner}/${repo.name}`}
          className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200"
        >
          {repo.name}
        </Link>
      ))}
    </div>
  );
}
