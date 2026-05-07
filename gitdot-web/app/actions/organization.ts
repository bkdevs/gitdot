"use server";

import type { OrganizationResource } from "gitdot-api";
import { refresh } from "next/cache";
import { createOrganization } from "@/dal/organization";

export type CreateOrganizationActionResult =
  | { organization: OrganizationResource }
  | { error: string };

export async function createOrganizationAction(
  formData: FormData,
): Promise<CreateOrganizationActionResult> {
  const name = formData.get("org-name") as string;

  if (!name) {
    return { error: "Organization name is required" };
  }

  const result = await createOrganization(name);
  if (!result) {
    return { error: "Failed to create organization" };
  }

  refresh();
  return { organization: result };
}
