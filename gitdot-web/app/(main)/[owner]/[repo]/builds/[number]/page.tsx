import {
  getBuild,
  getRepositoryCommit,
  getRepositoryFile,
  NotFound,
} from "@/dal";
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

  const data = await getBuild(owner, repo, number);
  if (!data) return null;

  const { build, tasks } = data;

  const commit = await getRepositoryCommit(owner, repo, build.commit_sha);

  const configFile = await getRepositoryFile(owner, repo, {
    ref_name: build.commit_sha,
    path: ".gitdot-ci.toml",
  });
  const configHtml =
    configFile && configFile !== NotFound
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
      {tasks.map((task) => (
        <BuildTask key={task.id} task={task} />
      ))}
    </div>
  );
}
