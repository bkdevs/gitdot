"use client";

import type { TaskResource } from "gitdot-api";
import { Suspense, use } from "react";
import type {
  ResourcePromisesType,
  ResourceRequestsType,
} from "@/(main)/[owner]/[repo]/resources";
import { useResolvePromises } from "@/(main)/[owner]/[repo]/resources";
import type { S2Record } from "@/lib/s2/shared";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { BuildHeader } from "./ui/build-header";
import { BuildTask } from "./ui/build-task";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  requests,
  promises,
  tasks,
  tokens,
  taskLogs,
  configHtml,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
  tasks: TaskResource[];
  tokens: (string | null)[];
  taskLogs: S2Record[][];
  configHtml: string | null;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        buildPromise={resolvedPromises.build}
        commitPromise={resolvedPromises.commit}
        tasks={tasks}
        tokens={tokens}
        taskLogs={taskLogs}
        configHtml={configHtml}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  buildPromise,
  commitPromise,
  tasks,
  tokens,
  taskLogs,
  configHtml,
}: {
  owner: string;
  repo: string;
  buildPromise: Promise<Resources["build"]>;
  commitPromise: Promise<Resources["commit"]>;
  tasks: TaskResource[];
  tokens: (string | null)[];
  taskLogs: S2Record[][];
  configHtml: string | null;
}) {
  const build = use(buildPromise);
  const commit = use(commitPromise);
  if (!build) return null;

  return (
    <div className="flex flex-col w-full flex-1 min-w-0 overflow-y-auto scrollbar-thin">
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
          token={tokens[i] ?? ""}
        />
      ))}
    </div>
  );
}
