"use server";

import type {
  OrganizationMemberResource,
  OrganizationResource,
} from "gitdot-api";
import { refresh } from "next/cache";
import { ApiError } from "@/dal";
import {
  addOrganizationMember,
  createOrganization,
  updateOrganization,
  updateOrganizationMember,
  uploadOrganizationImage,
} from "@/dal/organization";

export type CreateOrganizationActionResult =
  | { organization: OrganizationResource }
  | { error: string };

export async function createOrganizationAction(
  _prev: CreateOrganizationActionResult | null,
  formData: FormData,
): Promise<CreateOrganizationActionResult> {
  const name = formData.get("org-name") as string;
  const readmeRaw = formData.get("org-readme") as string | null;
  const readme = (readmeRaw ?? "").trim() || undefined;

  if (!name) {
    return { error: "Organization name is required" };
  }

  try {
    const result = await createOrganization(name, readme);
    if (!result) {
      return { error: "Failed to create organization" };
    }

    refresh();
    return { organization: result };
  } catch (e) {
    return {
      error:
        e instanceof ApiError ? e.message : "Failed to create organization",
    };
  }
}

export type UpdateOrganizationActionResult =
  | { organization: OrganizationResource }
  | { error: string };

export async function updateOrganizationAction(
  orgName: string,
  formData: FormData,
): Promise<UpdateOrganizationActionResult> {
  const location = formData.get("location") as string | null;
  const readme = formData.get("readme") as string | null;
  const linksRaw = formData.get("links") as string | null;
  const links: string[] | undefined =
    linksRaw !== null ? JSON.parse(linksRaw) : undefined;
  const displayName = formData.get("display_name") as string | null;

  try {
    const result = await updateOrganization(orgName, {
      location,
      readme,
      links,
      display_name: displayName,
    });
    if (!result) return { error: "Failed to update organization" };

    refresh();
    return { organization: result };
  } catch (e) {
    return {
      error:
        e instanceof ApiError ? e.message : "Failed to update organization",
    };
  }
}

export type AddOrganizationMemberActionResult =
  | { member: OrganizationMemberResource }
  | { error: string };

export async function addOrganizationMemberAction(
  orgName: string,
  _prev: AddOrganizationMemberActionResult | null,
  formData: FormData,
): Promise<AddOrganizationMemberActionResult> {
  const userName = (formData.get("user_name") as string | null)?.trim();
  if (!userName) return { error: "Username is required" };

  const roleDescriptionRaw = (
    formData.get("role_description") as string | null
  )?.trim();
  const roleDescription = roleDescriptionRaw || null;

  try {
    const result = await addOrganizationMember(orgName, {
      user_name: userName,
      role: "member",
      role_description: roleDescription,
    });
    if (!result) return { error: "Failed to add member" };

    refresh();
    return { member: result };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to add member",
    };
  }
}

export type UpdateOrganizationMemberActionResult =
  | { member: OrganizationMemberResource }
  | { error: string };

export async function updateOrganizationMemberAction(
  orgName: string,
  memberId: string,
  _prev: UpdateOrganizationMemberActionResult | null,
  formData: FormData,
): Promise<UpdateOrganizationMemberActionResult> {
  const raw = (formData.get("role_description") as string | null)?.trim();
  const roleDescription = raw || null;

  try {
    const result = await updateOrganizationMember(orgName, memberId, {
      role_description: roleDescription,
    });
    if (!result) return { error: "Failed to update member" };

    refresh();
    return { member: result };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to update member",
    };
  }
}

export async function uploadOrganizationImageAction(
  orgName: string,
  file: File,
): Promise<{ success: true } | { error: string }> {
  if (!file || file.size === 0) {
    return { error: "No file provided" };
  } else if (file.size > 5 * 1024 * 1024) {
    return { error: "Image must be under 5 MB." };
  } else if (!["image/jpeg", "image/png", "image/webp"].includes(file.type)) {
    return { error: "Unsupported image type — use JPEG, PNG, or WebP." };
  }

  try {
    const ok = await uploadOrganizationImage(orgName, file);
    if (!ok) return { error: "Upload failed — please try again." };
    return { success: true };
  } catch (e) {
    console.error("uploadOrganizationImageAction failed:", e);
    const msg = e instanceof Error ? e.message : "Unknown error";
    return { error: `Upload failed: ${msg}` };
  }
}
