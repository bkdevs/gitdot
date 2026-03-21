"use client";

import { racePromises } from "@/util";
import type { ClientProvider } from "./client";
import { DatabaseProvider } from "./database";
import type {
  ResourceDefinition,
  ResourcePromises,
  ResourceRequests,
} from "./types";

export function fetchResources<T extends ResourceDefinition>(
  owner: string,
  repo: string,
  requests: ResourceRequests<T>,
  promises: ResourcePromises<T>,
): ResourcePromises<T> {
  const db = new DatabaseProvider(owner, repo);

  for (const key of Object.keys(requests)) {
    (promises[key as keyof T] as Promise<unknown>).then((value) =>
      db.write(requests[key].method, value),
    );
  }

  return raceRequests([db], requests, promises);
}

function raceRequests<T extends ResourceDefinition>(
  providers: ClientProvider[],
  requests: ResourceRequests<T>,
  promises: ResourcePromises<T>,
): ResourcePromises<T> {
  const result: Record<string, Promise<unknown>> = {};

  for (const key of Object.keys(requests)) {
    const replayed = providers.map((p) => p.replay(requests)[key]);
    result[key] = racePromises(promises[key as keyof T], ...replayed);
  }

  return result as ResourcePromises<T>;
}
