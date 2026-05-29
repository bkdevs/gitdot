"use server";

import type {
  CreateRepositoryCommitFilterRequest,
  RepositoryCommitFilterResource,
  RepositoryResource,
  UpdateRepositoryCommitFilterRequest,
  UpdateRepositoryRequest,
} from "gitdot-api";
import {
  ApiError,
  createRepository,
  createRepositoryCommitFilter,
  deleteRepository,
  deleteRepositoryCommitFilter,
  starRepository,
  unstarRepository,
  updateRepository,
  updateRepositoryCommitFilter,
} from "gitdot-client";
import { refresh } from "next/cache";

export type CreateRepositoryActionResult =
  | { repository: RepositoryResource }
  | { error: string };

export async function createRepositoryAction(
  _prev: CreateRepositoryActionResult | null,
  formData: FormData,
): Promise<CreateRepositoryActionResult> {
  const owner = formData.get("owner") as string;
  const name = formData.get("repo-name") as string;
  const visibility = formData.get("visibility") as string;
  const ownerType = (formData.get("owner_type") as string) || "user";
  const description =
    (formData.get("repo-description") as string | null)?.trim() || undefined;
  const initReadme = formData.get("init_readme") === "true";
  const gitignoreTemplate =
    (formData.get("gitignore_template") as string | null) || undefined;
  const licenseTemplate =
    (formData.get("license_template") as string | null) || undefined;

  if (!owner || !name) {
    return { error: "Owner and repository name are required" };
  }

  try {
    const result = await createRepository(owner, name, {
      owner_type: ownerType,
      visibility,
      description,
      init_readme: initReadme,
      gitignore_template: gitignoreTemplate as
        | "rust"
        | "node"
        | "python"
        | "go"
        | undefined,
      license_template: licenseTemplate as "mit" | "apache-2.0" | undefined,
    });
    if (!result) {
      return { error: "Failed to create repository" };
    }

    refresh();
    return { repository: result };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to create repository",
    };
  }
}

export type UpdateRepositoryActionResult =
  | { repository: RepositoryResource }
  | { error: string };

export async function updateRepositoryAction(
  owner: string,
  repo: string,
  payload: UpdateRepositoryRequest,
): Promise<UpdateRepositoryActionResult> {
  try {
    const repository = await updateRepository(owner, repo, payload);
    if (!repository) {
      return { error: "Failed to update repository" };
    }
    refresh();
    return { repository };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to update repository",
    };
  }
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

  return { success: true };
}

export type StarRepositoryActionResult = { success: true } | { error: string };

export async function starRepositoryAction(
  owner: string,
  repo: string,
): Promise<StarRepositoryActionResult> {
  try {
    await starRepository(owner, repo);
    return { success: true };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to star repository",
    };
  }
}

export async function unstarRepositoryAction(
  owner: string,
  repo: string,
): Promise<StarRepositoryActionResult> {
  try {
    await unstarRepository(owner, repo);
    return { success: true };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to unstar repository",
    };
  }
}

export type CommitFilterActionResult =
  | { filter: RepositoryCommitFilterResource }
  | { error: string };

export async function createRepositoryCommitFilterAction(
  owner: string,
  repo: string,
  payload: CreateRepositoryCommitFilterRequest,
): Promise<CommitFilterActionResult> {
  if (!payload.name.trim()) {
    return { error: "Name is required" };
  }

  try {
    const filter = await createRepositoryCommitFilter(owner, repo, payload);
    if (!filter) {
      return { error: "Failed to save filter" };
    }
    refresh();
    return { filter };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to save filter",
    };
  }
}

export async function updateRepositoryCommitFilterAction(
  owner: string,
  repo: string,
  filterId: string,
  payload: UpdateRepositoryCommitFilterRequest,
): Promise<CommitFilterActionResult> {
  if (!payload.name.trim()) {
    return { error: "Name is required" };
  }

  try {
    const filter = await updateRepositoryCommitFilter(
      owner,
      repo,
      filterId,
      payload,
    );
    if (!filter) {
      return { error: "Failed to update filter" };
    }
    refresh();
    return { filter };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to update filter",
    };
  }
}

export type DeleteCommitFilterActionResult =
  | { success: true }
  | { error: string };

export async function deleteRepositoryCommitFilterAction(
  owner: string,
  repo: string,
  filterId: string,
): Promise<DeleteCommitFilterActionResult> {
  try {
    await deleteRepositoryCommitFilter(owner, repo, filterId);
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to delete filter",
    };
  }

  refresh();
  return { success: true };
}
