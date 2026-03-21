import "server-only";

import { ApiProvider } from "./api";
import type { ResourceDefinition, ResourceResultType } from "./types";

export * from "./types";

export function fetchResources<T extends ResourceDefinition>(
  owner: string,
  repo: string,
  resources: T,
): ResourceResultType<T> {
  return new ApiProvider(owner, repo).fetch(resources);
}
