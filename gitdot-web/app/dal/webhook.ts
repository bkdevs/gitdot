import "server-only";

import {
  type CreateWebhookRequest,
  type UpdateWebhookRequest,
  WebhookResource,
} from "gitdot-api";
import { z } from "zod";
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
): Promise<WebhookResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/webhooks`,
  );

  return await handleResponse(response, z.array(WebhookResource));
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
