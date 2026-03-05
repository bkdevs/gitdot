import {
  getBuild,
  getBuildTasks,
  getRepositoryBlob,
  getRepositoryCommit,
  issueTaskToken,
  NotFound,
} from "@/dal";
import { getTaskLogs } from "@/lib/s2/server";
import { renderFileToHtml } from "../../util/hast";
import { BuildHeader } from "./ui/build-header";
import { BuildTask } from "./ui/build-task";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
}) {
  const { owner, repo, number: numberStr } = await params;
  const number = Number(numberStr);
  if (Number.isNaN(number)) return null;

  const [build, tasks] = await Promise.all([
    getBuild(owner, repo, number),
    getBuildTasks(owner, repo, number),
  ]);
  if (!build || !tasks) return null;

  const taskTokens = await Promise.all(
    tasks.map((task) => issueTaskToken(task.id)),
  );

  const [commit, configFile, taskLogs] = await Promise.all([
    getRepositoryCommit(owner, repo, build.commit_sha),
    getRepositoryBlob(owner, repo, {
      ref_name: build.commit_sha,
      path: ".gitdot-ci.toml",
    }),
    Promise.all(
      tasks.map((task, i) => {
        const token = taskTokens[i];
        return token
          ? getTaskLogs(token, owner, repo, task.id).catch(() => [])
          : Promise.resolve([]);
      }),
    ),
  ]);

  const configHtml =
    configFile && configFile !== NotFound && configFile.type === "file"
      ? await renderFileToHtml(configFile, "vitesse-light")
      : null;

  return (
    <div className="flex flex-col w-full">
      <BuildHeader
        build={build}
        commit={commit}
        tasks={tasks}
        configHtml={configHtml}
      />
      {tasks.map((task, i) => (
        <BuildTask
          key={task.id}
          task={task}
          logs={taskLogs[i]}
          owner={owner}
          repo={repo}
          token={taskTokens[i] ?? ""}
        />
      ))}
    </div>
  );
}
