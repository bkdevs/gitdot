"use client";

import type {
  RepositoryBlobPairResource,
  RepositoryBlobResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { openIdb, type RepositoryMetadata } from "../db";
import { fetchCommitBlobs } from "../diff/client";
import type { DiffData, DiffEntry } from "../diff/types";
import { createShikiWorker, createSyncWorker } from "../workers";
import type { ShikiRequest, ShikiResponse } from "../workers/shiki";
import type { SyncResponse } from "../workers/sync";
import { GitdotProvider } from "./types";

export class ClientProvider extends GitdotProvider {
  private static _instance: ClientProvider | null = null;
  static get instance(): ClientProvider {
    if (!ClientProvider._instance) {
      ClientProvider._instance = new ClientProvider();
    }
    return ClientProvider._instance;
  }

  private db = openIdb();
  private metadatas = new Map<string, RepositoryMetadata>();
  private paths = new Map<string, RepositoryPathsResource>();
  private commits = new Map<string, RepositoryCommitResource[]>();
  private hasts = new Map<string, Root>();
  private diffs = new Map<string, DiffData>();

  private syncWorker: SharedWorker | null = null;
  private shikiWorker: SharedWorker | null = null;
  private syncRequests = new Map<string, () => void>();
  private shikiBlobRequests = new Map<string, (hast: Root) => void>();
  private shikiDiffRequests = new Map<string, (data: DiffData) => void>();

  private constructor() {
    super();
    if (typeof SharedWorker === "undefined") return;

    this.syncWorker = createSyncWorker();
    this.syncWorker.port.start();
    this.syncWorker.port.onmessage = (e: MessageEvent<SyncResponse>) => {
      const resolve = this.syncRequests.get(e.data.id);
      if (!resolve) return;
      this.syncRequests.delete(e.data.id);
      resolve();
    };

    this.shikiWorker = createShikiWorker();
    this.shikiWorker.port.start();
    this.shikiWorker.port.onmessage = (e: MessageEvent<ShikiResponse>) => {
      const res = e.data;
      if (res.kind === "blob") {
        const resolve = this.shikiBlobRequests.get(res.id);
        if (!resolve) return;
        this.shikiBlobRequests.delete(res.id);
        resolve(res.hast);
      } else {
        const resolve = this.shikiDiffRequests.get(res.id);
        if (!resolve) return;
        this.shikiDiffRequests.delete(res.id);
        resolve(res.data);
      }
    };
  }

  syncRepo(owner: string, repo: string, forceRefresh = false): Promise<void> {
    let resolve!: () => void;
    const done = new Promise<void>((r) => {
      resolve = r;
    });
    const id = crypto.randomUUID();
    this.syncRequests.set(id, resolve);
    this.syncWorker?.port.postMessage({ id, owner, repo, forceRefresh });
    done.then(async () => {
      const metadata = await this.db.getMetadata(owner, repo);
      if (metadata) this.metadatas.set(`${owner}/${repo}`, metadata);
      this.getPaths(owner, repo);
      this.getCommits(owner, repo);
    });
    return done;
  }

  repoSynced(owner: string, repo: string): boolean {
    const metadata = this.metadatas.get(`${owner}/${repo}`);
    if (!metadata) return false;
    return (
      Date.now() - new Date(metadata.last_updated).getTime() < 5 * 60 * 1000
    );
  }

  async invalidate(): Promise<void> {
    this.metadatas.clear();
    this.paths.clear();
    this.commits.clear();
    this.hasts.clear();
    this.diffs.clear();
    await this.db.clear();
  }

  async getPaths(
    owner: string,
    repo: string,
  ): Promise<RepositoryPathsResource | null> {
    if (!this.repoSynced(owner, repo)) return null;
    const key = `${owner}/${repo}`;
    const cached = this.paths.get(key);
    if (cached) return cached;
    const paths = await this.db.getPaths(owner, repo);
    if (paths) this.paths.set(key, paths);
    return paths;
  }

  async getCommits(
    owner: string,
    repo: string,
  ): Promise<RepositoryCommitResource[] | null> {
    if (!this.repoSynced(owner, repo)) return null;
    const key = `${owner}/${repo}`;
    const cached = this.commits.get(key);
    if (cached) return cached;
    const commits = await this.db.getCommits(owner, repo);
    if (commits === null || commits.length === 0) return null;
    const sorted = commits.sort(
      (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime(),
    );
    this.commits.set(key, sorted);
    return sorted;
  }

  async getBlob(
    owner: string,
    repo: string,
    path: string,
    _ref?: string,
  ): Promise<RepositoryBlobResource | null> {
    if (!this.repoSynced(owner, repo)) return null;
    return this.db.getBlob(owner, repo, path);
  }

  async getHast(
    owner: string,
    repo: string,
    path: string,
    _ref?: string,
  ): Promise<Root | null> {
    if (!this.repoSynced(owner, repo)) return null;
    const key = `${owner}/${repo}/${path}`;
    const cached = this.hasts.get(key);
    if (cached) return cached;

    const blob = await this.db.getBlob(owner, repo, path);
    if (!blob) return null;

    const id = crypto.randomUUID();
    const hast = await new Promise<Root>((resolve) => {
      this.shikiBlobRequests.set(id, resolve);
      this.shikiWorker?.port.postMessage({
        id,
        kind: "blob",
        path,
        content: blob.content,
      } satisfies ShikiRequest);
    });
    this.hasts.set(key, hast);
    return hast;
  }

  async getCommit(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<RepositoryCommitResource | null> {
    if (!this.repoSynced(owner, repo)) return null;
    const cached = this.commits.get(`${owner}/${repo}`);
    const hit = cached?.find((c) => c.sha === sha || c.sha.startsWith(sha));
    if (hit) return hit;
    return this.db.getCommit(owner, repo, sha);
  }

  async getCommitDiff(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<DiffData> {
    const key = `${owner}/${repo}/${sha}`;
    const cached = this.diffs.get(key);
    if (cached) return cached;

    const pairs = await fetchCommitBlobs(owner, repo, sha);
    const data = await new Promise<DiffData>((resolve) => {
      const id = crypto.randomUUID();
      this.shikiDiffRequests.set(id, resolve);
      this.shikiWorker?.port.postMessage({
        id,
        kind: "diff",
        pairs,
      } satisfies ShikiRequest);
    });
    this.diffs.set(key, data);
    return data;
  }

  async renderBlob(
    old: RepositoryBlobResource | null,
    next: RepositoryBlobResource | null,
  ): Promise<DiffEntry> {
    const path = old?.path ?? next?.path ?? "";
    const pair: RepositoryBlobPairResource = {
      path,
      old: old ?? undefined,
      new: next ?? undefined,
    };
    const data = await new Promise<DiffData>((resolve) => {
      const id = crypto.randomUUID();
      this.shikiDiffRequests.set(id, resolve);
      this.shikiWorker?.port.postMessage({
        id,
        kind: "diff",
        pairs: [pair],
      } satisfies ShikiRequest);
    });
    return data[0];
  }
}
