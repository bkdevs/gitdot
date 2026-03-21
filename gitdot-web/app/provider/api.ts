import "server-only";

import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";
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

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export class ApiProvider extends ServerProvider {
  async getBlob(path: string): Promise<RepositoryBlobResource | null> {
    await sleep(3000);
    return await getRepositoryBlob(this.owner, this.repo, { path: path });
  }

  async getCommit(sha: string): Promise<RepositoryCommitResource | null> {
    await sleep(3000);
    return await getRepositoryCommit(this.owner, this.repo, sha);
  }

  async getPaths(): Promise<RepositoryPathsResource | null> {
    await sleep(3000);
    return await getRepositoryPaths(this.owner, this.repo);
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    await sleep(3000);
    const result = await getRepositoryCommits(this.owner, this.repo, {
      from: subtractDays(new Date(), 365).toISOString(),
    });
    return result ? result.commits : null;
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    await sleep(3000);
    const paths = await getRepositoryPaths(this.owner, this.repo);
    if (!paths) return null;
    return await getRepositoryBlobs(this.owner, this.repo, {
      paths: paths.entries.map((e) => e.path),
    });
  }

  async getSettings(): Promise<RepositorySettingsResource | null> {
    await sleep(3000);
    return await getRepositorySettings(this.owner, this.repo);
  }
}
