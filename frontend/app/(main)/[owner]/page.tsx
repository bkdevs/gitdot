import { listUserRepositories } from "@/lib/dal";
import { UserRepos } from "./ui/user-repos";

export default async function RepoPage({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  const repos = await listUserRepositories(owner);
  if (!repos) return null;

  return (
    <div className="flex flex-col">
      <UserRepos user={owner} repos={repos} />
    </div>
  );
}
