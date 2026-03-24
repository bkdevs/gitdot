import "server-only";

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
import { getRepositoryHast } from "@/actions/repository";
import {
  getBuild as dalGetBuild,
  getBuilds as dalGetBuilds,
} from "@/dal/build";
import { listQuestions } from "@/dal/question";
import {
  getRepositoryBlob,
  getRepositoryBlobs,
  getRepositoryCommit,
  getRepositoryCommits,
  getRepositoryPaths,
  getRepositorySettings,
} from "@/dal/repository";
import { getReview as dalGetReview, listReviews } from "@/dal/review";
import { subtractDays } from "@/util/date";
import type { ResourceDefinition } from "./types";
import { ServerProvider } from "./types";

export function fetchResources<T extends ResourceDefinition>(
  owner: string,
  repo: string,
  resources: T,
) {
  return new ApiProvider(owner, repo).fetch(resources);
}

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

  async getQuestions(): Promise<QuestionResource[] | null> {
    return await listQuestions(this.owner, this.repo);
  }

  async getReview(number: number): Promise<ReviewResource | null> {
    return await dalGetReview(this.owner, this.repo, number);
  }

  async getReviews(): Promise<ReviewResource[] | null> {
    return await listReviews(this.owner, this.repo);
  }

  async getBuilds(): Promise<BuildResource[] | null> {
    return await dalGetBuilds(this.owner, this.repo);
  }

  async getBuild(number: number): Promise<BuildResource | null> {
    return await dalGetBuild(this.owner, this.repo, number);
  }
}
