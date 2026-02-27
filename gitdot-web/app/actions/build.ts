"use server";

import type { BuildResource } from "gitdot-api";
import { refresh } from "next/cache";
import { ApiError, createBuild } from "@/dal";

export type CreateBuildActionResult =
  | { build: BuildResource }
  | { error: string };

export async function createBuildAction(
  owner: string,
  repo: string,
  formData: FormData,
): Promise<CreateBuildActionResult> {
  const trigger = formData.get("trigger") as string;
  const commit_sha = formData.get("commit_sha") as string;

  if (!trigger || !commit_sha) {
    return { error: "Trigger and commit SHA are required" };
  }

  if (trigger !== "pull_request" && trigger !== "push_to_main") {
    return { error: "Trigger must be pull_request or push_to_main" };
  }

  let result: BuildResource | null;
  try {
    result = await createBuild(owner, repo, {
      trigger,
      commit_sha,
    });
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "createBuild call failed",
    };
  }
  if (!result) {
    return { error: "createBuild call failed" };
  }

  refresh();
  return { build: result };
}
