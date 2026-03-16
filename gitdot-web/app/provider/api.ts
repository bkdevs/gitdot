import "server-only";

import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import {
  getRepositoryBlob,
  getRepositoryBlobs,
  getRepositoryCommit,
  getRepositoryCommits,
  getRepositoryPaths,
} from "@/dal/repository";
import { RepoProvider } from "./types";

export class ApiProvider extends RepoProvider {
  async getBlob(path: string): Promise<RepositoryBlobResource | null> {
    return await getRepositoryBlob(this.owner, this.repo, { path: path });
  }

  async getCommit(sha: string): Promise<RepositoryCommitResource | null> {
    return await getRepositoryCommit(this.owner, this.repo, sha);
  }

  async getPaths(): Promise<RepositoryPathsResource | null> {
    return await getRepositoryPaths(this.owner, this.repo);
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    const result = await getRepositoryCommits(this.owner, this.repo);
    return result ? result.commits : null;
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    const paths = await getRepositoryPaths(this.owner, this.repo);
    if (!paths) return null;
    return await getRepositoryBlobs(this.owner, this.repo, {
      paths: paths.entries.map((e) => e.path),
    });
  }
}
