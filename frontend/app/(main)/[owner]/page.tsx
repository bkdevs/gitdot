import { listUserRepositories, NotFound } from "@/lib/dal";
import { UserRepos } from "./ui/user-repos";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  const repos = await listUserRepositories(owner);
  if (!repos) return null;

  return (
    <div className="flex flex-col">
      {repos === NotFound ? (
        <p className="p-2 text-sm">User {owner} not found</p>
      ) : (
        <UserRepos user={owner} repos={repos} />
      )}
    </div>
  );
}
