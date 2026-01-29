import { listUserRepositories } from "@/lib/dal";
import Link from "@/ui/link";

export default async function RepoPage({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  const repos = await listUserRepositories(owner);
  if (!repos) return null;

  return (
    <div className="flex flex-col w-full">
      {repos.map((repo) => (
        <Link
          className="h-9 flex flex-row text-sm font-mono items-center px-2 border-b"
          key={repo.id}
          href={`/${owner}/${repo.name}`}
        >
          {repo.name}
        </Link>
      ))}
    </div>
  );
}
