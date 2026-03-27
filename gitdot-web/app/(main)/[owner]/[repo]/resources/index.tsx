"use client";

export type {
  ResourcePromisesType,
  ResourceRequestsType,
} from "@/provider/types";

import { DatabaseProvider } from "@/provider/database";
import type {
  ClientProvider,
  ResourcePromisesType,
  ResourceRequestsType,
} from "@/provider/types";
import { racePromises } from "@/util";
import { useRepoContext } from "./context";

export function useResolvePromises<S>(
  owner: string,
  repo: string,
  requests: ResourceRequestsType<S>,
  promises: ResourcePromisesType<S>,
): ResourcePromisesType<S> {
  const { provider: memoryProvider } = useRepoContext();
  const dbProvider = new DatabaseProvider(owner, repo);

  for (const key of Object.keys(requests)) {
    promises[key as keyof S].then((value) =>
      dbProvider.write(
        requests[key as keyof S].method,
        requests[key as keyof S].args,
        value,
      ),
    );
  }

  return raceRequests([memoryProvider, dbProvider], requests, promises);
}

function raceRequests<S>(
  providers: ClientProvider[],
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
