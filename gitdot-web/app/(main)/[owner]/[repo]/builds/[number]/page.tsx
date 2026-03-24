import type { BuildResource, RepositoryCommitResource } from "gitdot-api";
import {
  getBuild,
  getBuildTasks,
  getRepositoryBlob,
  issueTaskToken,
} from "@/dal";
import { getTaskLogs } from "@/lib/s2/server";
import { fetchResources } from "@/provider/server";
import { renderFileToHtml } from "../../util/hast";
import { PageClient } from "./page.client";

export type Resources = {
  build: BuildResource | null;
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

  const { requests, promises } = fetchResources(owner, repo, {
    build: (p) => p.getBuild(number),
    commit: (p) => p.getCommit(build.commit_sha),
  });

  const tasks = await getBuildTasks(owner, repo, number);
  if (!tasks) return null;

  const tokens = await Promise.all(
    tasks.map((task) => issueTaskToken(task.id)),
  );

  const [configFile, taskLogs] = await Promise.all([
    getRepositoryBlob(owner, repo, {
      ref_name: build.commit_sha,
      path: ".gitdot-ci.toml",
    }),
    Promise.all(
      tasks.map((task, i) => {
        const token = tokens[i];
        return token
          ? getTaskLogs(token, owner, repo, task.id).catch(() => [])
          : Promise.resolve([]);
      }),
    ),
  ]);

  const configHtml =
    configFile && configFile.type === "file"
      ? await renderFileToHtml(configFile, "vitesse-light")
      : null;

  return (
    <PageClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
      tasks={tasks}
      tokens={tokens}
      taskLogs={taskLogs}
      configHtml={configHtml}
    />
  );
}
