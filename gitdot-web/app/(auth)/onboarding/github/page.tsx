import { listInstallationRepositories } from "@/dal";
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
    const repositories = await listInstallationRepositories(
      Number(installation_id),
    );
    return <RepositorySelect repositories={repositories} />;
  }

  return <GitHubImport />;
}
