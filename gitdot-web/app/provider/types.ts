import type {
  BuildResource,
  QuestionResource,
  RepositoryBlobResource,
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  ReviewResource,
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

export type ShapeFromDefinition<T extends ResourceDefinition> = {
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
  abstract getCommits(): Promise<RepositoryCommitResource[] | null>;
  abstract getBlob(
    path: string,
    ref?: string,
  ): Promise<RepositoryBlobResource | null>;
  abstract getHast(path: string, ref?: string): Promise<Root | null>;
  abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>;
  abstract getCommitFilters(): Promise<RepositoryCommitFilterResource[] | null>;
  abstract getQuestions(): Promise<QuestionResource[] | null>;
  abstract getReview(number: number): Promise<ReviewResource | null>;
  abstract getReviews(): Promise<ReviewResource[] | null>;
  abstract getBuilds(): Promise<BuildResource[] | null>;
  abstract getBuild(number: number): Promise<BuildResource | null>;
}
