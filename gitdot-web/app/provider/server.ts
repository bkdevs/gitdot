import "server-only";
import {
  RepoProvider,
  type ResourceDefinition,
  type ResourceRequest,
  type ResourceResult,
} from "./types";

export abstract class ServerProvider extends RepoProvider {
  fetch<T extends ResourceDefinition>(def: T): ResourceResult<T> {
    const promises: Record<string, Promise<unknown>> = {};
    const requests: Record<string, ResourceRequest> = {};

    for (const [key, factory] of Object.entries(def)) {
      let request: ResourceRequest | null = null;

      const proxy = new Proxy(this, {
        get(target, prop: string) {
          const func = target[prop as keyof typeof target];
          if (typeof func !== "function") {
            throw new Error("Provider.fetch should only invoke methods");
          } else if (request) {
            throw new Error(
              "Provider.fetch should only invoke a single method",
            );
          }

          return (...args: unknown[]) => {
            request = { method: prop, args };
            return func.apply(target, args);
          };
        },
      });

      const promise = factory(proxy);
      if (!request) {
        throw new Error("Provider.fetch did not invoke any methods");
      }

      promises[key] = promise;
      requests[key] = request;
    }

    return { promises, requests } as ResourceResult<T>;
  }
}
