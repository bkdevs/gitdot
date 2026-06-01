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
  listRepositoryCommits,
} from "gitdot-client";
import type { Root } from "hast";
import { renderCommitDiff } from "../diff/server";
import { inferLanguage, renderHast } from "../diff/shiki";
import type { DiffData } from "../diff/types";
import {
  GitdotProvider,
  type ResourceDefinition,
  type ResourceRequestType,
  type ResourceResultType,
  type ShapeFromDefinition,
} from "./types";

export class ServerProvider extends GitdotProvider {
  fetch<T extends ResourceDefinition>(
    def: T,
  ): ResourceResultType<ShapeFromDefinition<T>> {
    const promises: Record<string, Promise<unknown>> = {};
    const requests: Record<string, ResourceRequestType> = {};

    for (const [key, factory] of Object.entries(def)) {
      let request: ResourceRequestType | null = null;

      const proxy = new Proxy(this, {
        get(target, prop: string) {
          const func = target[prop as keyof typeof target];
          if (typeof func !== "function") {
            throw new Error("Provider.fetch should only invoke methods");
          } else if (request) {
            throw new Error(
              "Provider.fetch should only invoke a single method",
            );
          }

          return (...args: unknown[]) => {
            request = { method: prop, args };
            return func.apply(target, args);
          };
        },
      });

      const promise = factory(proxy);
      if (!request) {
        throw new Error("Provider.fetch did not invoke any methods");
      }

      promises[key] = promise;
      requests[key] = request;
    }

    return { promises, requests } as ResourceResultType<ShapeFromDefinition<T>>;
  }

  async getPaths(
    owner: string,
    repo: string,
  ): Promise<RepositoryPathsResource | null> {
    return await getRepositoryPaths(owner, repo);
  }

  async getCommits(
    owner: string,
    repo: string,
  ): Promise<RepositoryCommitResource[] | null> {
    const result = await listRepositoryCommits(owner, repo);
    return result?.data ?? null;
  }

  async getBlob(
    owner: string,
    repo: string,
    path: string,
    ref?: string,
  ): Promise<RepositoryBlobResource | null> {
    return await getRepositoryBlob(owner, repo, {
      path,
      ...(ref && { ref_name: ref }),
    });
  }

  async getHast(
    owner: string,
    repo: string,
    path: string,
    ref?: string,
  ): Promise<Root | null> {
    const blob = await getRepositoryBlob(owner, repo, {
      path,
      ...(ref && { ref_name: ref }),
    });
    if (!blob) return null;
    const lang = inferLanguage(path);
    return renderHast(blob.content, lang, "gitdot");
  }

  async getCommit(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<RepositoryCommitResource | null> {
    return await getRepositoryCommit(owner, repo, sha);
  }

  async getCommitDiff(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<DiffData> {
    return await renderCommitDiff(owner, repo, sha);
  }
}
