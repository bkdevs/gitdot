import { notFound } from "next/navigation";
import {
  getCurrentUser,
  listInstallationRepositories,
  listInstallations,
} from "@/dal";
import { GitHubImport } from "./ui/github-import";
import { RepositorySelect } from "./ui/repository-select";

export default async function Page({
  searchParams,
}: {
  searchParams: Promise<Record<string, string | string[] | undefined>>;
}) {
  const params = await searchParams;
  const installation_id =
    typeof params.installation_id === "string"
      ? params.installation_id
      : undefined;

  if (installation_id) {
    const [user, installations, repositories] = await Promise.all([
      getCurrentUser(),
      listInstallations(),
      listInstallationRepositories(Number(installation_id)),
    ]);
    if (!user) notFound();

    const installation = (installations ?? []).find(
      (i) => i.installation_id === Number(installation_id),
    );
    if (!installation) notFound();

    return (
      <RepositorySelect
        user={user}
        installation={installation}
        repositories={repositories}
      />
    );
  }

  return <GitHubImport />;
}
