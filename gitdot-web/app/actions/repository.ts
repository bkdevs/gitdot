"use server";

import type {
  DiffHunkResource,
  RepositoryDiffResource,
  RepositoryResource,
} from "gitdot-api";
import type { Element } from "hast";
import { refresh } from "next/cache";
import { redirect } from "next/navigation";
import {
  createChangeMaps,
  fileToHast,
  mergeHunks,
  renderSpans,
} from "@/(main)/[owner]/[repo]/util";
import {
  ApiError,
  createRepository,
  deleteRepository,
  getRepositoryBlob,
  getRepositoryCommit,
  migrateGitHubRepositories,
} from "@/dal";

export type DiffData =
  | {
      kind: "split";
      leftSpans: Element[];
      rightSpans: Element[];
      hunks: DiffHunkResource[];
    }
  | { kind: "single"; spans: Element[] }
  | { kind: "no-change" };

const NULL_SHA = "0000000000000000000000000000000000000000";

async function computeDiffData(
  owner: string,
  repo: string,
  diffs: RepositoryDiffResource[],
  sha: string,
  parentSha: string,
): Promise<Record<string, DiffData>> {
  const results = await Promise.all(
    diffs.map(async (stat) => {
      const [rightBlob, leftBlob] = await Promise.all([
        getRepositoryBlob(owner, repo, { ref_name: sha, path: stat.path }),
        parentSha !== NULL_SHA
          ? getRepositoryBlob(owner, repo, {
              ref_name: parentSha,
              path: stat.path,
            })
          : null,
      ]);

      const right = rightBlob?.type === "file" ? rightBlob : null;
      const left = leftBlob?.type === "file" ? leftBlob : null;
      const processedHunks = mergeHunks(stat.hunks);

      if (left && right && stat.hunks.length > 0) {
        const { leftChangeMap, rightChangeMap } =
          createChangeMaps(processedHunks);
        const [leftSpans, rightSpans] = await Promise.all([
          renderSpans("left", left, leftChangeMap),
          renderSpans("right", right, rightChangeMap),
        ]);
        return {
          kind: "split" as const,
          leftSpans,
          rightSpans,
          hunks: processedHunks,
        };
      } else if (left || right) {
        // biome-ignore lint/style/noNonNullAssertion: guaranteed non-null by the `left || right` condition
        const file = (left ?? right)!;
        const side = left ? "left" : "right";
        const lineType = side === "left" ? "removed" : "added";
        const hast = await fileToHast(file, "vitesse-light", [
          {
            line(node, lineNumber) {
              node.type = "element";
              node.tagName = "diffline";
              node.properties["data-line-number"] = lineNumber;
              node.properties["data-line-type"] = lineType;
            },
          },
        ]);
        const pre = hast.children[0] as Element;
        const code = pre.children[0] as Element;
        const spans = code.children.filter(
          (child): child is Element => child.type === "element",
        );
        return { kind: "single" as const, spans };
      } else {
        return { kind: "no-change" as const };
      }
    }),
  );

  return Object.fromEntries(diffs.map((stat, i) => [stat.path, results[i]]));
}

export async function getAllDiffDataAction(
  owner: string,
  repo: string,
  sha: string,
): Promise<Record<string, DiffData>> {
  const commit = await getRepositoryCommit(owner, repo, sha);
  if (!commit) return {};
  return computeDiffData(owner, repo, commit.diffs, sha, commit.parent_sha);
}

export async function getReviewAllDiffDataAction(
  owner: string,
  repo: string,
  files: RepositoryDiffResource[],
  sha: string,
  parentSha: string,
): Promise<Record<string, DiffData>> {
  return computeDiffData(owner, repo, files, sha, parentSha);
}

export type CreateRepositoryActionResult =
  | { repository: RepositoryResource }
  | { error: string };

export async function createRepositoryAction(
  formData: FormData,
): Promise<CreateRepositoryActionResult> {
  const owner = formData.get("owner") as string;
  const name = formData.get("repo-name") as string;
  const visibility = formData.get("visibility") as string;

  if (!owner || !name) {
    return { error: "Owner and repository name are required" };
  }

  const result = await createRepository(owner, name, {
    owner_type: "user",
    visibility,
  });
  if (!result) {
    return { error: "Failed to create repository" };
  }

  refresh();
  return { repository: result };
}

export type DeleteRepositoryActionResult =
  | { success: true }
  | { error: string };

export async function deleteRepositoryAction(
  owner: string,
  repo: string,
): Promise<DeleteRepositoryActionResult> {
  try {
    await deleteRepository(owner, repo);
  } catch (e) {
    return {
      error: e instanceof Error ? e.message : "Failed to delete repository",
    };
  }

  redirect(`/${owner}`);
  return { success: true };
}

export type MigrateGitHubRepositoriesActionResult =
  | { success: true }
  | { error: string };

export async function migrateGitHubRepositoriesAction(
  installationId: number,
  origin: string,
  originType: string,
  destination: string,
  destinationType: string,
  repositories: string[],
): Promise<MigrateGitHubRepositoriesActionResult> {
  if (!destination || repositories.length === 0) {
    return { error: "Destination and repositories are required" };
  }

  try {
    await migrateGitHubRepositories(
      installationId,
      origin,
      originType,
      destination,
      destinationType,
      repositories,
    );
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to start migration",
    };
  }

  redirect("/settings/migrations");
  return { success: true };
}
