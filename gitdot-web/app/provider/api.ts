"use server";

import {
  getRepositoryBlob,
  getRepositoryCommit,
  getRepositoryPaths,
} from "@/dal/repository";
import type {
  RepositoryBlobResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { RepoProvider } from "./types";

export class ApiProvider extends RepoProvider {
  async getBlob(id: string): Promise<RepositoryBlobResource | null> {
    return await getRepositoryBlob(this.owner, this.repo, { path: id });
  }

  async getCommit(sha: string): Promise<RepositoryCommitResource | null> {
    return await getRepositoryCommit(this.owner, this.repo, sha);
  }

  async getPaths(): Promise<RepositoryPathsResource | null> {
    return await getRepositoryPaths(this.owner, this.repo);
  }
}
