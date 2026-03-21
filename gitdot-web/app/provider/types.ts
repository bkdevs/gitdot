import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";

type ResourcesDef = Record<
  string,
  (provider: RepoProvider) => Promise<unknown>
>;
type ResourcesResult<T extends ResourcesDef> = {
  [K in keyof T]: ReturnType<T[K]>;
}

export abstract class RepoProvider {
  protected owner: string;
  protected repo: string;

  constructor(owner: string, repo: string) {
    this.owner = owner;
    this.repo = repo;
  }

  fetch<T extends ResourcesDef>(def: T): ResourcesResult<T> {
    const promises: Record<string, Promise<unknown>> = {};

    for (const [key, factory] of Object.entries(def)) {
      promises[key] = factory(this);
    }

    return promises as ResourcesResult<T>;
  }

  abstract getPaths(): Promise<RepositoryPathsResource | null>;
  abstract getBlob(path: string): Promise<RepositoryBlobResource | null>;
  abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>;
  abstract getCommits(): Promise<RepositoryCommitResource[] | null>;
  abstract getBlobs(): Promise<RepositoryBlobsResource | null>;
  abstract getSettings(): Promise<RepositorySettingsResource | null>;
}
