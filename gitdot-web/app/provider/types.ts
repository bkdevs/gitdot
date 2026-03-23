import type {
  QuestionResource,
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";
import type { Root } from "hast";

export type ResourceDefinition = Record<
  string,
  (provider: RepoProvider) => Promise<unknown>
>;

export type ResourceRequestType = { method: string; args: unknown[] };
export type ResourceRequestsType<S> = {
  [K in keyof S]: ResourceRequestType;
};

export type ResourcePromisesType<S> = {
  [K in keyof S]: Promise<S[K]>;
};

export type ResourceResult<S> = {
  promises: ResourcePromisesType<S>;
  requests: ResourceRequestsType<S>;
};

type ShapeFromDefinition<T extends ResourceDefinition> = {
  [K in keyof T]: Awaited<ReturnType<T[K]>>;
};

export abstract class RepoProvider {
  protected owner: string;
  protected repo: string;

  constructor(owner: string, repo: string) {
    this.owner = owner;
    this.repo = repo;
  }

  abstract getPaths(): Promise<RepositoryPathsResource | null>;
  abstract getBlob(path: string): Promise<RepositoryBlobResource | null>;
  abstract getHast(path: string): Promise<Root | null>;
  abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>;
  abstract getCommits(): Promise<RepositoryCommitResource[] | null>;
  abstract getBlobs(): Promise<RepositoryBlobsResource | null>;
  abstract getSettings(): Promise<RepositorySettingsResource | null>;
  abstract getQuestions(): Promise<QuestionResource[] | null>;
}

export abstract class ServerProvider extends RepoProvider {
  fetch<T extends ResourceDefinition>(
    def: T,
  ): ResourceResult<ShapeFromDefinition<T>> {
    const promises: Record<string, Promise<unknown>> = {};
    const requests: Record<string, ResourceRequestType> = {};

    for (const [key, factory] of Object.entries(def)) {
      let request: ResourceRequestType | null = null;

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

    return { promises, requests } as ResourceResult<ShapeFromDefinition<T>>;
  }
}

export abstract class ClientProvider extends RepoProvider {
  replay(
    requests: Record<string, ResourceRequestType>,
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
