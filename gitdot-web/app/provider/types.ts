import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";

type MethodCall = { method: string; args: any[] };

export abstract class RepoProvider {
  protected owner: string;
  protected repo: string;
  private promises: Map<MethodCall, Promise<any>> = new Map();

  constructor(owner: string, repo: string) {
    this.owner = owner;
    this.repo = repo;

    return new Proxy(this, {
      get(target, prop) {
        const val = (target as any)[prop];
        if (typeof val !== 'function' || prop === 'getPromises') return val;
        return (...args: any[]) => {
          const call: MethodCall = { method: prop as string, args };
          const promise = val.apply(target, args);
          target.promises.set(call, promise);
          return promise;
        };
      }
    });
  }

  getPromises(): Map<MethodCall, Promise<any>> {
    return this.promises;
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
