import "server-only";

import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { getRepositoryHast } from "@/actions/repository";
import {
  getRepositoryBlob,
  getRepositoryBlobs,
  getRepositoryCommit,
  getRepositoryCommits,
  getRepositoryPaths,
  getRepositorySettings,
} from "@/dal/repository";
import { subtractDays } from "@/util/date";
import { ServerProvider } from "./server";

export class ApiProvider extends ServerProvider {
  async getBlob(path: string): Promise<RepositoryBlobResource | null> {
    return await getRepositoryBlob(this.owner, this.repo, { path: path });
  }

  async getHast(path: string): Promise<Root | null> {
    return await getRepositoryHast(this.owner, this.repo, path);
  }

  async getCommit(sha: string): Promise<RepositoryCommitResource | null> {
    return await getRepositoryCommit(this.owner, this.repo, sha);
  }

  async getPaths(): Promise<RepositoryPathsResource | null> {
    return await getRepositoryPaths(this.owner, this.repo);
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    const result = await getRepositoryCommits(this.owner, this.repo, {
      from: subtractDays(new Date(), 365).toISOString(),
    });
    return result ? result.commits : null;
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    const paths = await getRepositoryPaths(this.owner, this.repo);
    if (!paths) return null;
    return await getRepositoryBlobs(this.owner, this.repo, {
      paths: paths.entries.map((e) => e.path),
    });
  }

  async getSettings(): Promise<RepositorySettingsResource | null> {
    return await getRepositorySettings(this.owner, this.repo);
  }
}
