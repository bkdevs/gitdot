"use server";

import { redirect } from "next/navigation";
import {
  authorizeDevice,
  createRunner,
  deleteRunner,
  refreshRunnerToken,
} from "@/dal";

export type CreateRunnerActionResult = { error: string };

export async function createRunnerAction(
  _prev: CreateRunnerActionResult | null,
  formData: FormData,
): Promise<CreateRunnerActionResult> {
  const name = formData.get("name") as string;
  const ownerName = formData.get("owner_name") as string;
  const ownerType = formData.get("owner_type") as string;

  if (!name || !ownerName || !ownerType) {
    return { error: "Name, owner, and owner type are required" };
  }

  if (ownerType !== "user" && ownerType !== "organization") {
    return { error: "Owner type must be user or organization" };
  }

  const result = await createRunner(name, ownerName, ownerType);
  if (!result) {
    return { error: "Failed to create runner" };
  }

  if (ownerType === "organization") {
    redirect(`/${ownerName}/settings/runners/${result.name}`);
  }
  redirect(`/settings/runners/${result.name}`);
}

export type RefreshRunnerTokenActionResult =
  | { token: string }
  | { error: string };

export async function refreshRunnerTokenAction(
  runnerName: string,
  ownerName: string,
): Promise<RefreshRunnerTokenActionResult> {
  if (!runnerName || !ownerName) {
    return { error: "Runner name and owner are required" };
  }

  const result = await refreshRunnerToken(ownerName, runnerName);
  if (!result) {
    return { error: "Failed to generate token" };
  }

  return { token: result.token };
}

export type AuthorizeDeviceActionResult =
  | { success: true }
  | { success: false; error: string };

export async function authorizeDeviceAction(
  userCode: string,
): Promise<AuthorizeDeviceActionResult> {
  if (!userCode) {
    return { success: false, error: "User code is required" };
  }

  const success = await authorizeDevice({ user_code: userCode });
  if (!success) {
    return { success: false, error: "Failed to authorize device" };
  }

  return { success: true };
}

export async function deleteRunnerAction(
  ownerName: string,
  runnerName: string,
  ownerType = "user",
): Promise<never> {
  await deleteRunner(ownerName, runnerName);
  if (ownerType === "organization") {
    redirect(`/${ownerName}/settings/runners`);
  }
  redirect("/settings/runners");
}
