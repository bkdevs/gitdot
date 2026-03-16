import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";

type ContextDef = Record<string, (provider: RepoProvider) => Promise<any>>;
type ContextResult<T extends ContextDef> = {
  [K in keyof T]: ReturnType<T[K]>;
} & {
  promises: Map<string, Promise<any>>;
};

export abstract class RepoProvider {
  protected owner: string;
  protected repo: string;

  constructor(owner: string, repo: string) {
    this.owner = owner;
    this.repo = repo;
  }

  define<T extends ContextDef>(def: T): ContextResult<T> {
    const promises = new Map<string, Promise<any>>();
    const context: Record<string, Promise<any>> = {};

    for (const [key, factory] of Object.entries(def)) {
      const promise = factory(this);
      promises.set(key, promise);
      context[key] = promise;
    }

    return {
      ...context,
      promises,
    } as ContextResult<T>;
  }

  abstract getBlob(path: string): Promise<RepositoryBlobResource | null>;
  abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>;
  abstract getPaths(): Promise<RepositoryPathsResource | null>;
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
