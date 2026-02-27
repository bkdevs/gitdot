import { getBuild, getRepositoryCommit } from "@/dal";
import { BuildHeader } from "./ui/build-header";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
}) {
  const { owner, repo, number: numberStr } = await params;
  const number = Number(numberStr);
  if (Number.isNaN(number)) return null;

  const data = await getBuild(owner, repo, number);
  if (!data) return null;

  const { build, tasks } = data;

  const commit = await getRepositoryCommit(owner, repo, build.commit_sha);

  return <BuildHeader build={build} commit={commit} tasks={tasks} />;
}
