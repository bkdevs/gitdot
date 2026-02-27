import { listUserOrganizations, listUserRepositories, NotFound } from "@/dal";
import { UserOrgs } from "./ui/user-orgs";
import { UserRepos } from "./ui/user-repos";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  const [repos, orgs] = await Promise.all([
    listUserRepositories(owner),
    listUserOrganizations(owner),
  ]);
  if (!repos) return null;

  return (
    <div className="flex flex-col">
      {repos === NotFound ? (
        <p className="p-2 text-sm">User {owner} not found</p>
      ) : (
        <>
          {orgs && orgs !== NotFound && orgs.length > 0 && (
            <UserOrgs orgs={orgs} />
          )}
          <UserRepos user={owner} repos={repos} />
        </>
      )}
    </div>
  );
}
