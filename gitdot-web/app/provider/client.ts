import { RepoProvider, type ResourceRequest } from "./types";

export abstract class ClientProvider extends RepoProvider {
  replay(
    requests: Record<string, ResourceRequest>,
  ): Record<string, Promise<unknown>> {
    const results: Record<string, Promise<unknown>> = {};
    for (const [key, { method, args }] of Object.entries(requests)) {
      const func = this[method as keyof this];
      if (typeof func !== "function") {
        throw new Error(`ClientProvider has no method "${method}"`);
      }
      results[key] = func.apply(this, args);
    }
    return results;
  }
}
