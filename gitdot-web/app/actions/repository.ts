"use server";

import type {
  CreateRepositoryCommitFilterRequest,
  RepositoryCommitFilterResource,
  RepositoryResource,
  UpdateRepositoryCommitFilterRequest,
} from "gitdot-api";
import type { Root } from "hast";
import { refresh } from "next/cache";
import { redirect } from "next/navigation";
import { fileToHast, inferLanguage } from "@/(main)/[owner]/[repo]/util";
import {
  ApiError,
  convertReadonlyRepository,
  createRepository,
  createRepositoryCommitFilter,
  deleteRepository,
  deleteRepositoryCommitFilter,
  getRepositoryBlob,
  starRepository,
  unstarRepository,
  updateRepositoryCommitFilter,
} from "@/dal";

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

export async function getRepositoryHastAction(
  owner: string,
  repo: string,
  path: string,
  ref?: string,
): Promise<Root | null> {
  const blob = await getRepositoryBlob(owner, repo, {
    path,
    ...(ref && { ref_name: ref }),
  });
  if (!blob || blob.type === "folder") return null;
  const lang = inferLanguage(path);
  return fileToHast(blob.content, lang, "vitesse", []);
}

export type StarRepositoryActionResult = { success: true } | { error: string };

export async function starRepositoryAction(
  owner: string,
  repo: string,
): Promise<StarRepositoryActionResult> {
  try {
    await starRepository(owner, repo);
    refresh();
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
    refresh();
    return { success: true };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to unstar repository",
    };
  }
}

export type ConvertReadonlyRepositoryActionResult =
  | { repository: RepositoryResource }
  | { error: string };

export async function convertReadonlyRepositoryAction(
  owner: string,
  repo: string,
): Promise<ConvertReadonlyRepositoryActionResult> {
  try {
    const repository = await convertReadonlyRepository(owner, repo);
    if (!repository) {
      return { error: "Failed to convert repository" };
    }
    refresh();
    return { repository };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to convert repository",
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
