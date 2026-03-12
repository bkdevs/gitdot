"use client";

import type {
  RepositoryCommitResource,
  RepositoryPreviewResource,
  RepositoryTreeResource,
} from "gitdot-api";

export interface Database {
  getCommit(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<RepositoryCommitResource | undefined>;

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

  getTree(
    owner: string,
    repo: string,
  ): Promise<RepositoryTreeResource | undefined>;

  putTree(
    owner: string,
    repo: string,
    tree: RepositoryTreeResource,
  ): Promise<void>;

  getPreview(
    owner: string,
    repo: string,
  ): Promise<RepositoryPreviewResource | undefined>;

  putPreview(
    owner: string,
    repo: string,
    preview: RepositoryPreviewResource,
  ): Promise<void>;
}
