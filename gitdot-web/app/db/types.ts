import { type RepositoryCommitResource } from "gitdot-api";

export interface RepositoryCommitRow extends RepositoryCommitResource {
  owner: string;
  repo: string;
}

export interface Database {
  getCommit(sha: string): Promise<RepositoryCommitRow | undefined>;
  putCommit(commit: RepositoryCommitRow): Promise<void>;
  putCommits(commits: RepositoryCommitRow[]): Promise<void>;
  getAllCommits(): Promise<RepositoryCommitRow[]>;
}
