import type { RepositoryCommitResource } from "gitdot-api";
import { getBuild, getBuildTasks, issueTaskToken } from "gitdot-client";
import { fetchResources } from "gitdot-dal/server";
import { getTaskLogs } from "@/lib/s2/server";
import { PageClient } from "./page.client";

export type Resources = {
  commit: RepositoryCommitResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
}) {
  const { owner, repo, number: numberStr } = await params;
  const number = Number(numberStr);
  if (Number.isNaN(number)) return null;

  const build = await getBuild(owner, repo, number);
  if (!build) return null;

  const resources = fetchResources({
    commit: (p) => p.getCommit(owner, repo, build.commit_sha),
  });

  const tasks = await getBuildTasks(owner, repo, number);
  if (!tasks) return null;

  const tokens = await Promise.all(
    tasks.map((task) => issueTaskToken(task.id)),
  );

  const taskLogs = await Promise.all(
    tasks.map((task, i) => {
      const token = tokens[i];
      return token
        ? getTaskLogs(token, owner, repo, task.id).catch(() => [])
        : Promise.resolve([]);
    }),
  );

  return (
    <PageClient
      owner={owner}
      repo={repo}
      resources={resources}
      build={build}
      tasks={tasks}
      tokens={tokens}
      taskLogs={taskLogs}
    />
  );
}
