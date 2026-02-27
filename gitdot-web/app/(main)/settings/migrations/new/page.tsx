import { notFound } from "next/navigation";
import {
  getCurrentUser,
  listInstallationRepositories,
  listInstallations,
  listUserOrganizations,
  NotFound,
} from "@/dal";
import { CreateMigrationForm } from "./ui/create-migration-form";
import { CreateMigrationInstructions } from "./ui/create-migration-instructions";

export default async function Page({
  searchParams,
}: {
  searchParams: Promise<{ installation_id?: string }>;
}) {
  const user = await getCurrentUser();
  if (!user) notFound();

  const params = await searchParams;
  const [orgs, installations] = await Promise.all([
    listUserOrganizations(user.name),
    listInstallations(),
  ]);
  const organizations = orgs && orgs !== NotFound ? orgs : [];
  const installationList = installations ?? [];
  const defaultOrigin = installationList.find(
    (i) => i.installation_id === Number(params.installation_id),
  )?.github_login;

  const reposByInstallation = Object.fromEntries(
    await Promise.all(
      installationList.map(async (i) => [
        i.github_login,
        (await listInstallationRepositories(i.installation_id)) ?? [],
      ]),
    ),
  );

  return (
    <div className="flex p-4">
      <div className="min-w-0 flex-1 max-w-3xl">
        <CreateMigrationForm
          user={user}
          organizations={organizations}
          installations={installationList}
          reposByInstallation={reposByInstallation}
          defaultOrigin={defaultOrigin}
        />
      </div>
      <aside className="w-72 shrink-0 hidden md:block border-l border-border ml-6 pl-6">
        <CreateMigrationInstructions />
      </aside>
    </div>
  );
}
