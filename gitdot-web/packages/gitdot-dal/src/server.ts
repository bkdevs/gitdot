import "server-only";

import { ServerProvider } from "./provider/server";
import type { ResourceDefinition } from "./provider/types";

export * from "./diff/server";
export * from "./diff/shiki";
export * from "./diff/types";
export * from "./provider/server";
export * from "./provider/types";

export function fetchResources<T extends ResourceDefinition>(resources: T) {
  return new ServerProvider().fetch(resources);
}
