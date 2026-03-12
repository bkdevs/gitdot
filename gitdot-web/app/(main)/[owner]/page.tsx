import { listUserOrganizations, listUserRepositories } from "@/dal";
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
  return (
    <div className="flex flex-col">
      {repos === null ? (
        <p className="p-2 text-sm">User {owner} not found</p>
      ) : (
        <>
          {orgs && orgs.length > 0 && <UserOrgs orgs={orgs} />}
          <UserRepos user={owner} repos={repos} />
        </>
      )}
    </div>
  );
}
