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

  abstract getBlob(path: string): Promise<RepositoryBlobResource | null>;
  abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>;
  abstract getPaths(): Promise<RepositoryPathsResource | null>;
  abstract getCommits(): Promise<RepositoryCommitResource[] | null>;
  abstract getBlobs(): Promise<RepositoryBlobsResource | null>;
}

export interface Database {
  getCommit(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<RepositoryCommitResource | null>;

  putCommit(
    owner: string,
    repo: string,
    commit: RepositoryCommitResource,
  ): Promise<void>;

  putCommits(
    owner: string,
    repo: string,
    commits: RepositoryCommitResource[],
  ): Promise<void>;

  getAllCommits(
    owner: string,
    repo: string,
  ): Promise<RepositoryCommitResource[]>;

  getPaths(
    owner: string,
    repo: string,
  ): Promise<RepositoryPathsResource | null>;

  putPaths(
    owner: string,
    repo: string,
    paths: RepositoryPathsResource,
  ): Promise<void>;

  getBlob(
    owner: string,
    repo: string,
    path: string,
  ): Promise<RepositoryBlobResource | null>;

  getBlobs(
    owner: string,
    repo: string,
  ): Promise<RepositoryBlobsResource | undefined>;

  putBlobs(
    owner: string,
    repo: string,
    blobs: RepositoryBlobsResource,
  ): Promise<void>;
}
