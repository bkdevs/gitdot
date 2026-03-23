import type {
  QuestionResource,
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
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

  getHast(owner: string, repo: string, path: string): Promise<Root | null>;

  getHasts(owner: string, repo: string): Promise<Map<string, Root> | null>;

  putHast(owner: string, repo: string, path: string, hast: Root): Promise<void>;

  getSettings(
    owner: string,
    repo: string,
  ): Promise<RepositorySettingsResource | null>;

  putSettings(
    owner: string,
    repo: string,
    settings: RepositorySettingsResource,
  ): Promise<void>;

  getQuestions(
    owner: string,
    repo: string,
  ): Promise<QuestionResource[] | null>;

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
}
