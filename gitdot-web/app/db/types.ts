import type {
  BuildResource,
  QuestionResource,
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
  ReviewResource,
} from "gitdot-api";
import type { Root } from "hast";

export type RepositoryMetadata = {
  last_commit: string;
  last_updated: string;
};

export interface Database {
  getPaths(
    owner: string,
    repo: string,
  ): Promise<RepositoryPathsResource | null>;

  putPaths(
    owner: string,
    repo: string,
    paths: RepositoryPathsResource,
  ): Promise<void>;

  getCommit(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<RepositoryCommitResource | null>;

  getCommits(owner: string, repo: string): Promise<RepositoryCommitResource[]>;

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

  getBlob(
    owner: string,
    repo: string,
    path: string,
    commit: string,
  ): Promise<RepositoryBlobResource | null>;

  getBlobs(
    owner: string,
    repo: string,
    commit: string,
  ): Promise<RepositoryBlobsResource | null>;

  putBlobs(
    owner: string,
    repo: string,
    blobs: RepositoryBlobsResource,
  ): Promise<void>;

  getHast(
    owner: string,
    repo: string,
    path: string,
    commit: string,
  ): Promise<Root | null>;

  getHasts(
    owner: string,
    repo: string,
    commit: string,
  ): Promise<Map<string, Root> | null>;

  putHast(
    owner: string,
    repo: string,
    path: string,
    hast: Root,
    commit: string,
  ): Promise<void>;

  getSettings(
    owner: string,
    repo: string,
  ): Promise<RepositorySettingsResource | null>;

  putSettings(
    owner: string,
    repo: string,
    settings: RepositorySettingsResource,
  ): Promise<void>;

  getQuestions(owner: string, repo: string): Promise<QuestionResource[] | null>;

  putQuestions(
    owner: string,
    repo: string,
    questions: QuestionResource[],
  ): Promise<void>;

  getMetadata(owner: string, repo: string): Promise<RepositoryMetadata | null>;

  putMetadata(
    owner: string,
    repo: string,
    metadata: RepositoryMetadata,
  ): Promise<void>;

  getReview(
    owner: string,
    repo: string,
    number: number,
  ): Promise<ReviewResource | null>;

  getReviews(owner: string, repo: string): Promise<ReviewResource[]>;

  putReview(
    owner: string,
    repo: string,
    number: number,
    review: ReviewResource,
  ): Promise<void>;

  getBuilds(owner: string, repo: string): Promise<BuildResource[] | null>;
  putBuilds(
    owner: string,
    repo: string,
    builds: BuildResource[],
  ): Promise<void>;
  getBuild(
    owner: string,
    repo: string,
    number: number,
  ): Promise<BuildResource | null>;
  putBuild(owner: string, repo: string, build: BuildResource): Promise<void>;
}
