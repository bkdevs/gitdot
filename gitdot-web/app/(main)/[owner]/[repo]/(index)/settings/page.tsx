import { notFound } from "next/navigation";

import { getCurrentUser, listRunners, listWebhooks } from "@/dal";
import { RepositorySettingsGeneral } from "./ui/repository-settings-general";
import { RepositorySettingsRunners } from "./ui/repository-settings-runners";
import { RepositorySettingsWebhooks } from "./ui/repository-settings-webhooks";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;

  const current = await getCurrentUser(false);
  const isAdmin =
    current?.user.name === owner ||
    (current?.memberships ?? []).some(
      (m) => m.org_name === owner && m.role === "admin",
    );
  if (!isAdmin) notFound();

  const [runners, webhooksResponse] = await Promise.all([
    listRunners(owner),
    listWebhooks(owner, repo),
  ]);
  const webhooks = webhooksResponse?.data ?? null;

  return (
    <div className="flex flex-col w-full">
      <RepositorySettingsGeneral owner={owner} repo={repo} />
      <RepositorySettingsWebhooks
        owner={owner}
        repo={repo}
        webhooks={webhooks ?? []}
      />
      <RepositorySettingsRunners runners={runners ?? []} />
    </div>
  );
}
