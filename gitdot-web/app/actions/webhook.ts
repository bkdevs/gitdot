"use server";

import type { WebhookResource } from "gitdot-api";
import { refresh } from "next/cache";
import { ApiError, createWebhook, deleteWebhook, updateWebhook } from "@/dal";

export type CreateWebhookActionResult =
  | { webhook: WebhookResource }
  | { error: string };

export async function createWebhookAction(
  owner: string,
  repo: string,
  formData: FormData,
): Promise<CreateWebhookActionResult> {
  const url = formData.get("url") as string;
  const secret = formData.get("secret") as string;
  const events = formData.getAll("events") as string[];

  if (!url) {
    return { error: "URL is required" };
  }
  if (!secret) {
    return { error: "Secret is required" };
  }
  if (events.length === 0) {
    return { error: "At least one event is required" };
  }

  try {
    const result = await createWebhook(owner, repo, { url, secret, events });
    if (!result) {
      return { error: "Failed to create webhook" };
    }

    refresh();
    return { webhook: result };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to create webhook",
    };
  }
}

export type DeleteWebhookActionResult = { success: true } | { error: string };

export async function deleteWebhookAction(
  owner: string,
  repo: string,
  webhookId: string,
): Promise<DeleteWebhookActionResult> {
  try {
    await deleteWebhook(owner, repo, webhookId);
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to delete webhook",
    };
  }

  refresh();
  return { success: true };
}

export type UpdateWebhookActionResult =
  | { webhook: WebhookResource }
  | { error: string };

export async function updateWebhookAction(
  owner: string,
  repo: string,
  webhookId: string,
  formData: FormData,
): Promise<UpdateWebhookActionResult> {
  const url = formData.get("url") as string | null;
  const events = formData.getAll("events") as string[];

  try {
    const result = await updateWebhook(owner, repo, webhookId, {
      url: url || undefined,
      events: events.length > 0 ? events : undefined,
    });
    if (!result) {
      return { error: "Failed to update webhook" };
    }

    refresh();
    return { webhook: result };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to update webhook",
    };
  }
}
