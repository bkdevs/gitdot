import { getBuilds } from "@/dal";
import { BuildsClient } from "./ui/builds-client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const builds = await getBuilds(owner, repo);
  if (!builds) return null;

  return <BuildsClient owner={owner} repo={repo} builds={builds} />;
}
