"use client";

import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositoryPreviewResource,
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

  getPreview(
    owner: string,
    repo: string,
  ): Promise<RepositoryPreviewResource | undefined>;

  putPreview(
    owner: string,
    repo: string,
    preview: RepositoryPreviewResource,
  ): Promise<void>;

  getPaths(
    owner: string,
    repo: string,
  ): Promise<RepositoryPathsResource | undefined>;

  putPaths(
    owner: string,
    repo: string,
    paths: RepositoryPathsResource,
  ): Promise<void>;

  getBlob(
    owner: string,
    repo: string,
    path: string,
  ): Promise<RepositoryBlobResource | undefined>;

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
