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
import type { RepositoryMetadata } from "@/db/types";
import { ClientProvider } from "./types";

export class DatabaseProvider extends ClientProvider {
  private db = openIdb();
  private metadataPromise: Promise<RepositoryMetadata | null> | null = null;

  private metadata() {
    this.metadataPromise ??= this.db.getMetadata(this.owner, this.repo);
    return this.metadataPromise;
  }

  async getPaths() {
    return this.db.getPaths(this.owner, this.repo);
  }

  async putPaths(paths: RepositoryPathsResource) {
    return this.db.putPaths(this.owner, this.repo, paths);
  }

  async getBlob(path: string, ref?: string) {
    if (ref) return this.db.getBlob(this.owner, this.repo, path, ref);
    const metadata = await this.metadata();
    if (!metadata) return null;
    return this.db.getBlob(this.owner, this.repo, path, metadata.last_commit);
  }

  async getHast(path: string, ref?: string): Promise<Root | null> {
    if (ref) return this.db.getHast(this.owner, this.repo, path, ref);
    const metadata = await this.metadata();
    if (!metadata) return null;
    return this.db.getHast(this.owner, this.repo, path, metadata.last_commit);
  }

  async putHast(path: string, hast: Root, commit: string): Promise<void> {
    return this.db.putHast(this.owner, this.repo, path, hast, commit);
  }

  async getCommit(sha: string) {
    return this.db.getCommit(this.owner, this.repo, sha);
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    const commits = await this.db.getCommits(this.owner, this.repo);
    if (commits === null || commits.length === 0) return null;
    return commits.sort(
      (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime(),
    );
  }

  async putCommits(commits: RepositoryCommitResource[]) {
    return this.db.putCommits(this.owner, this.repo, commits);
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    const metadata = await this.metadata();
    if (!metadata) return null;
    return await this.db.getBlobs(this.owner, this.repo, metadata.last_commit);
  }

  async putBlobs(blobs: RepositoryBlobsResource) {
    return this.db.putBlobs(this.owner, this.repo, blobs);
  }

  async putMetadata(metadata: RepositoryMetadata) {
    this.metadataPromise = Promise.resolve(metadata);
    return this.db.putMetadata(this.owner, this.repo, metadata);
  }

  async getSettings() {
    return this.db.getSettings(this.owner, this.repo);
  }

  async putSettings(settings: RepositorySettingsResource) {
    return this.db.putSettings(this.owner, this.repo, settings);
  }

  async getQuestions(): Promise<QuestionResource[] | null> {
    return this.db.getQuestions(this.owner, this.repo);
  }

  async putQuestions(questions: QuestionResource[]) {
    return this.db.putQuestions(this.owner, this.repo, questions);
  }

  async getReview(number: number): Promise<ReviewResource | null> {
    return this.db.getReview(this.owner, this.repo, number);
  }

  async getReviews(): Promise<ReviewResource[] | null> {
    const reviews = await this.db.getReviews(this.owner, this.repo);
    if (reviews.length === 0) return null;
    return reviews;
  }

  async putReview(number: number, review: ReviewResource): Promise<void> {
    return this.db.putReview(this.owner, this.repo, number, review);
  }

  async deleteReview(number: number): Promise<void> {
    return this.db.deleteReview(this.owner, this.repo, number);
  }

  async getBuilds(): Promise<BuildResource[] | null> {
    return this.db.getBuilds(this.owner, this.repo);
  }

  async putBuilds(builds: BuildResource[]): Promise<void> {
    return this.db.putBuilds(this.owner, this.repo, builds);
  }

  async getBuild(number: number): Promise<BuildResource | null> {
    return this.db.getBuild(this.owner, this.repo, number);
  }

  async putBuild(_number: number, build: BuildResource): Promise<void> {
    return this.db.putBuild(this.owner, this.repo, build);
  }

  async putBlob(path: string, commit: string, blob: RepositoryBlobResource) {
    return this.db.putBlob(this.owner, this.repo, path, commit, blob);
  }

  // TODO: this is a tad hacky (relying on the fact that provider get / put methods) are serialized as such
  // we can do better if we discrminate resource types directly with a "type" field.
  private writers: Record<string, (args: unknown[], value: unknown) => void> = {
    getPaths: (_args, v) => this.putPaths(v as RepositoryPathsResource),
    getCommits: (_args, v) => this.putCommits(v as RepositoryCommitResource[]),
    getBlobs: (_args, v) => this.putBlobs(v as RepositoryBlobsResource),
    getSettings: (_args, v) =>
      this.putSettings(v as RepositorySettingsResource),
    getQuestions: (_args, v) => this.putQuestions(v as QuestionResource[]),
    getReview: (_args, v) =>
      this.putReview((v as ReviewResource).number, v as ReviewResource),
    getReviews: (_args, v) => {
      for (const r of v as ReviewResource[]) this.putReview(r.number, r);
    },
    getBuilds: (_args, v) => this.putBuilds(v as BuildResource[]),
    getBuild: (_args, v) =>
      this.putBuild((v as BuildResource).number, v as BuildResource),
    getBlob: (args, v) => {
      if (!args[1]) return;
      this.putBlob(
        args[0] as string,
        args[1] as string,
        v as RepositoryBlobResource,
      );
    },
    getHast: (args, v) => {
      const ref = args[1] as string | undefined;
      if (!ref) return;
      this.putHast(args[0] as string, v as Root, ref);
    },
  };

  write(method: string, args: unknown[], value: unknown) {
    if (!value) return;
    this.writers[method]?.(args, value);
  }
}
