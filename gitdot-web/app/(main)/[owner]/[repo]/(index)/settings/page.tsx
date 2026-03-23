import { notFound } from "next/navigation";

import { listRunners } from "@/dal";
import { getUserMetadata } from "@/lib/supabase";
import { RepositorySettingsGeneral } from "./ui/repository-settings-general";
import { RepositorySettingsRunners } from "./ui/repository-settings-runners";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;

  const { username, orgs } = await getUserMetadata();
  const isAdmin = username === owner || orgs.includes(`${owner}:admin`);
  if (!isAdmin) notFound();

  const runners = (await listRunners(owner)) ?? [];

  return (
    <div className="flex flex-col w-full">
      <RepositorySettingsGeneral owner={owner} repo={repo} />
      <RepositorySettingsRunners runners={runners} />
    </div>
  );
}
