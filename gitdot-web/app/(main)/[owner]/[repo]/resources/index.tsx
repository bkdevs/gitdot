"use client";

export type {
  ResourcePromisesType,
  ResourceRequestsType,
  ResourceResultType,
} from "@/provider/types";

import { LocalProvider } from "@/provider/local";
import type {
  ResourcePromisesType,
  ResourceRequestsType,
  ResourceResultType,
} from "@/provider/types";
import { racePromises } from "@/util";

export function useResolvePromises<S>(
  owner: string,
  repo: string,
  resources: ResourceResultType<S>,
): ResourcePromisesType<S> {
  const local = new LocalProvider(owner, repo);
  return raceRequests([local], resources.requests, resources.promises);
}

function raceRequests<S>(
  providers: LocalProvider[],
  requests: ResourceRequestsType<S>,
  promises: ResourcePromisesType<S>,
): ResourcePromisesType<S> {
  const result: Record<string, Promise<unknown>> = {};

  for (const key of Object.keys(requests)) {
    const replayed = providers.map((p) => p.replay(requests)[key]);
    result[key] = racePromises(promises[key as keyof S], ...replayed);
  }

  return result as ResourcePromisesType<S>;
}
