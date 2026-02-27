"use server";

import type { RepositoryResource } from "gitdot-api";
import { refresh } from "next/cache";
import { redirect } from "next/navigation";
import {
  ApiError,
  createRepository,
  deleteRepository,
  migrateGitHubRepositories,
} from "@/dal";

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
