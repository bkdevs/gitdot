"use client";

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
import { openIdb } from "@/db";
import { ClientProvider } from "./types";

type Store = {
  paths: RepositoryPathsResource | undefined;
  blobs: RepositoryBlobsResource | undefined;
  commits: RepositoryCommitResource[] | undefined;
  settings: RepositorySettingsResource | undefined;
  questions: QuestionResource[] | undefined;
  builds: BuildResource[] | undefined;
  blob: Map<string, RepositoryBlobResource>;
  commit: Map<string, RepositoryCommitResource>;
  hast: Map<string, Root>;
  review: Map<number, ReviewResource>;
};

export class InMemoryProvider extends ClientProvider {
  private store: Store = {
    paths: undefined,
    blobs: undefined,
    commits: undefined,
    settings: undefined,
    questions: undefined,
    builds: undefined,
    blob: new Map(),
    commit: new Map(),
    hast: new Map(),
    review: new Map(),
  };

  async getPaths(): Promise<RepositoryPathsResource | null> {
    return this.store.paths ?? null;
  }

  async getBlob(path: string): Promise<RepositoryBlobResource | null> {
    return this.store.blob.get(path) ?? null;
  }

  async getHast(path: string): Promise<Root | null> {
    return this.store.hast.get(path) ?? null;
  }

  putHast(path: string, hast: Root): void {
    this.store.hast.set(path, hast);
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

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    return this.store.blobs ?? null;
  }

  async getSettings(): Promise<RepositorySettingsResource | null> {
    return this.store.settings ?? null;
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

  async initialize(): Promise<void> {
    const db = openIdb();
    const [paths, blobs, commits, settings, hasts, questions, reviews, builds] =
      await Promise.all([
        db.getPaths(this.owner, this.repo),
        db.getBlobs(this.owner, this.repo),
        db.getCommits(this.owner, this.repo),
        db.getSettings(this.owner, this.repo),
        db.getHasts(this.owner, this.repo),
        db.getQuestions(this.owner, this.repo),
        db.getReviews(this.owner, this.repo),
        db.getBuilds(this.owner, this.repo),
      ]);
    if (paths) this.store.paths = paths;
    if (blobs) {
      this.store.blobs = blobs;
      for (const b of blobs.blobs) this.store.blob.set(b.path, b);
    }
    if (commits?.length) {
      this.store.commits = commits;
      for (const c of commits) this.store.commit.set(c.sha.slice(0, 7), c);
    }
    if (settings) this.store.settings = settings;
    if (hasts) this.store.hast = hasts;
    if (questions?.length) this.store.questions = questions;
    if (reviews?.length) {
      for (const r of reviews) this.store.review.set(r.number, r);
    }
    if (builds?.length) this.store.builds = builds;
  }
}
