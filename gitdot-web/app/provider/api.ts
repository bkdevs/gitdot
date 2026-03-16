import "server-only";

import type {
  RepositoryBlobResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import {
  getRepositoryBlob,
  getRepositoryCommit,
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
}
