import { notFound } from "next/navigation";

import { listRunners, listWebhooks } from "@/dal";
import { getUserMetadata } from "@/lib/supabase";
import { RepositorySettingsGeneral } from "./ui/repository-settings-general";
import { RepositorySettingsRunners } from "./ui/repository-settings-runners";
import { RepositorySettingsWebhooks } from "./ui/repository-settings-webhooks";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;

  const { username, orgs } = await getUserMetadata();
  const isAdmin = username === owner || orgs.includes(`${owner}:admin`);
  if (!isAdmin) notFound();

  const [runners, webhooks] = await Promise.all([
    listRunners(owner),
    listWebhooks(owner, repo),
  ]);

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
