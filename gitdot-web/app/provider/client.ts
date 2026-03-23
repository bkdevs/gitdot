"use client";

import { racePromises } from "@/util";
import { DatabaseProvider } from "./database";
import type {
  ClientProvider,
  ResourcePromisesType,
  ResourceRequestsType,
} from "./types";

export * from "./types";

export function resolveResources<S>(
  owner: string,
  repo: string,
  requests: ResourceRequestsType<S>,
  promises: ResourcePromisesType<S>,
): ResourcePromisesType<S> {
  const db = new DatabaseProvider(owner, repo);

  for (const key of Object.keys(requests)) {
    promises[key as keyof S].then((value) =>
      db.write(requests[key as keyof S].method, value),
    );
  }

  return raceRequests([db], requests, promises);
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
