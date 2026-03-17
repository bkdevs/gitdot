import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";

type ResourcesDef = Record<
  string,
  (provider: RepoProvider) => Promise<unknown>
>;
type ResourcesResult<T extends ResourcesDef> = {
  [K in keyof T]: ReturnType<T[K]>;
} & {
  promises: Map<string, Promise<unknown>>;
};

export abstract class RepoProvider {
  protected owner: string;
  protected repo: string;

  constructor(owner: string, repo: string) {
    this.owner = owner;
    this.repo = repo;
  }

  fetch<T extends ResourcesDef>(def: T): ResourcesResult<T> {
    const promises = new Map<string, Promise<unknown>>();
    const context: Record<string, Promise<unknown>> = {};

    for (const [key, factory] of Object.entries(def)) {
      const promise = factory(this);
      promises.set(key, promise);
      context[key] = promise;
    }

    return {
      ...context,
      promises,
    } as ResourcesResult<T>;
  }

  abstract getPaths(): Promise<RepositoryPathsResource | null>;
  abstract getBlob(path: string): Promise<RepositoryBlobResource | null>;
  abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>;
  abstract getCommits(): Promise<RepositoryCommitResource[] | null>;
  abstract getBlobs(): Promise<RepositoryBlobsResource | null>;
}
