import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";

export type ResourceDefinition = Record<
  string,
  (provider: RepoProvider) => Promise<unknown>
>;

export type ResourceRequest = { method: string; args: unknown[] };
export type ResourceRequests<T extends ResourceDefinition> = {
  [K in keyof T]: ResourceRequest;
};

export type ResourcePromises<T extends ResourceDefinition> = {
  [K in keyof T]: Promise<Awaited<ReturnType<T[K]>>>;
};

export type ResourceResult<T extends ResourceDefinition> = {
  promises: ResourcePromises<T>;
  requests: ResourceRequests<T>;
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
  abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>;
  abstract getCommits(): Promise<RepositoryCommitResource[] | null>;
  abstract getBlobs(): Promise<RepositoryBlobsResource | null>;
  abstract getSettings(): Promise<RepositorySettingsResource | null>;
}
