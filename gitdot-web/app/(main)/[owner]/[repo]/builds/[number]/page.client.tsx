"use client";

import type { BuildResource, TaskResource } from "gitdot-api";
import { type ResourceResultType, useResources } from "gitdot-dal/client";
import { Suspense, use } from "react";
import type { S2Record } from "@/lib/s2/shared";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { BuildHeader } from "./ui/build-header";
import { BuildTask } from "./ui/build-task";

export function PageClient({
  owner,
  repo,
  resources,
  build,
  tasks,
  tokens,
  taskLogs,
}: {
  owner: string;
  repo: string;
  resources: ResourceResultType<Resources>;
  build: BuildResource;
  tasks: TaskResource[];
  tokens: (string | null)[];
  taskLogs: S2Record[][];
}) {
  const resourcePromises = useResources(resources);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        build={build}
        commitPromise={resourcePromises.commit}
        tasks={tasks}
        tokens={tokens}
        taskLogs={taskLogs}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  build,
  commitPromise,
  tasks,
  tokens,
  taskLogs,
}: {
  owner: string;
  repo: string;
  build: BuildResource;
  commitPromise: Promise<Resources["commit"]>;
  tasks: TaskResource[];
  tokens: (string | null)[];
  taskLogs: S2Record[][];
}) {
  const commit = use(commitPromise);

  return (
    <div className="flex flex-col w-full flex-1 min-w-0 overflow-y-auto scrollbar-thin">
      <BuildHeader build={build} commit={commit} tasks={tasks} />
      {tasks.map((task, i) => (
        <BuildTask
          key={task.id}
          task={task}
          logs={taskLogs[i]}
          owner={owner}
          repo={repo}
          token={tokens[i] ?? ""}
        />
      ))}
    </div>
  );
}
