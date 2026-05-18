import "server-only";

import {
  type CreateWebhookRequest,
  ListWebhooksResponse,
  type UpdateWebhookRequest,
  WebhookResource,
} from "gitdot-api";
import { toQueryString } from "@/util";
import {
  authDelete,
  authFetch,
  authPatch,
  authPost,
  GITDOT_SERVER_URL,
  handleResponse,
} from "./util";

export async function createWebhook(
  owner: string,
  repo: string,
  request: CreateWebhookRequest,
): Promise<WebhookResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/webhook`,
    request,
  );

  return await handleResponse(response, WebhookResource);
}

export async function listWebhooks(
  owner: string,
  repo: string,
  opts?: { cursor?: string; limit?: number },
): Promise<ListWebhooksResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/webhooks${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListWebhooksResponse);
}

export async function updateWebhook(
  owner: string,
  repo: string,
  webhookId: string,
  request: UpdateWebhookRequest,
): Promise<WebhookResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/webhook/${webhookId}`,
    request,
  );

  return await handleResponse(response, WebhookResource);
}

export async function deleteWebhook(
  owner: string,
  repo: string,
  webhookId: string,
): Promise<void> {
  await authDelete(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/webhook/${webhookId}`,
  );
}
