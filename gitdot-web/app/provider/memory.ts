"use client";

import type {
  BuildResource,
  QuestionResource,
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  ReviewResource,
} from "gitdot-api";
import type { Root } from "hast";
import { openIdb } from "@/db";
import { ClientProvider } from "./types";

type Store = {
  paths: RepositoryPathsResource | undefined;
  commits: RepositoryCommitResource[] | undefined;
  questions: QuestionResource[] | undefined;
  builds: BuildResource[] | undefined;
  commit: Map<string, RepositoryCommitResource>;
  review: Map<number, ReviewResource>;
  build: Map<number, BuildResource>;
};

export class InMemoryProvider extends ClientProvider {
  private store: Store = {
    paths: undefined,
    commits: undefined,
    questions: undefined,
    builds: undefined,
    commit: new Map(),
    review: new Map(),
    build: new Map(),
  };

  async getPaths(): Promise<RepositoryPathsResource | null> {
    return this.store.paths ?? null;
  }

  async getBlob(
    _path: string,
    _ref?: string,
  ): Promise<RepositoryBlobResource | null> {
    return null;
  }

  async getHast(_path: string, _ref?: string): Promise<Root | null> {
    return null;
  }

  async getCommit(sha: string): Promise<RepositoryCommitResource | null> {
    const result = this.store.commit.get(sha) ?? null;
    return result;
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    if (!this.store.commits) return null;
    return this.store.commits.sort(
      (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime(),
    );
  }

  async getCommitFilters(): Promise<RepositoryCommitFilterResource[] | null> {
    return null;
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    return null;
  }

  async getQuestions(): Promise<QuestionResource[] | null> {
    return this.store.questions ?? null;
  }

  async getReview(number: number): Promise<ReviewResource | null> {
    return this.store.review.get(number) ?? null;
  }

  async getReviews(): Promise<ReviewResource[] | null> {
    if (this.store.review.size === 0) return null;
    return Array.from(this.store.review.values());
  }

  async getBuilds(): Promise<BuildResource[] | null> {
    return this.store.builds ?? null;
  }

  async getBuild(number: number): Promise<BuildResource | null> {
    return this.store.build.get(number) ?? null;
  }

  async initialize(): Promise<void> {
    const db = openIdb();
    const paths = await db.getPaths(this.owner, this.repo);
    const commits = await db.getCommits(this.owner, this.repo);
    const questions = await db.getQuestions(this.owner, this.repo);
    const reviews = await db.getReviews(this.owner, this.repo);
    const builds = await db.getBuilds(this.owner, this.repo);

    if (paths) this.store.paths = paths;
    if (commits?.length) {
      this.store.commits = commits;
      for (const c of commits) this.store.commit.set(c.sha.slice(0, 7), c);
    }
    if (questions?.length) this.store.questions = questions;
    if (reviews?.length) {
      for (const r of reviews) this.store.review.set(r.number, r);
    }
    if (builds?.length) {
      this.store.builds = builds;
      for (const b of builds) this.store.build.set(b.number, b);
    }
  }
}
