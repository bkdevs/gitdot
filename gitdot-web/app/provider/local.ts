"use client";

import type {
  BuildResource,
  QuestionResource,
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  ReviewResource,
} from "gitdot-api";
import type { Root } from "hast";
import { openIdb } from "@/db";
import { RepoProvider, type ResourceRequestType } from "./types";

const pathsCache = new Map<string, RepositoryPathsResource>();
const commitsCache = new Map<string, RepositoryCommitResource[]>();

function repoKey(owner: string, repo: string) {
  return `${owner}/${repo}`;
}

export class LocalProvider extends RepoProvider {
  private db = openIdb();

  replay(
    requests: Record<string, ResourceRequestType>,
  ): Record<string, Promise<unknown>> {
    const results: Record<string, Promise<unknown>> = {};
    for (const [key, { method, args }] of Object.entries(requests)) {
      const func = this[method as keyof this];
      if (typeof func !== "function") {
        throw new Error(`LocalProvider has no method "${method}"`);
      }
      results[key] = func.apply(this, args);
    }
    return results;
  }

  async initialize() {
    await Promise.all([this.getPaths(), this.getCommits()]);
  }

  async getPaths() {
    const key = repoKey(this.owner, this.repo);
    const cached = pathsCache.get(key);
    if (cached) return cached;
    const paths = await this.db.getPaths(this.owner, this.repo);
    if (paths) pathsCache.set(key, paths);
    return paths;
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    const key = repoKey(this.owner, this.repo);
    const cached = commitsCache.get(key);
    if (cached) return cached;
    const commits = await this.db.getCommits(this.owner, this.repo);
    if (commits === null || commits.length === 0) return null;
    const sorted = commits.sort(
      (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime(),
    );
    commitsCache.set(key, sorted);
    return sorted;
  }

  async getBlob(path: string, _ref?: string) {
    return this.db.getBlob(this.owner, this.repo, path);
  }

  async getHast(path: string, _ref?: string): Promise<Root | null> {
    return this.db.getHast(this.owner, this.repo, path);
  }

  async getCommit(sha: string) {
    const cached = commitsCache.get(repoKey(this.owner, this.repo));
    const hit = cached?.find((c) => c.sha === sha || c.sha.startsWith(sha));
    if (hit) return hit;
    return this.db.getCommit(this.owner, this.repo, sha);
  }

  async getCommitFilters(): Promise<RepositoryCommitFilterResource[] | null> {
    return null;
  }

  async getQuestions(): Promise<QuestionResource[] | null> {
    return this.db.getQuestions(this.owner, this.repo);
  }

  async getReview(number: number): Promise<ReviewResource | null> {
    return this.db.getReview(this.owner, this.repo, number);
  }

  async getReviews(): Promise<ReviewResource[] | null> {
    const reviews = await this.db.getReviews(this.owner, this.repo);
    return reviews.length === 0 ? null : reviews;
  }

  async getBuilds(): Promise<BuildResource[] | null> {
    return this.db.getBuilds(this.owner, this.repo);
  }

  async getBuild(number: number): Promise<BuildResource | null> {
    return this.db.getBuild(this.owner, this.repo, number);
  }
}
