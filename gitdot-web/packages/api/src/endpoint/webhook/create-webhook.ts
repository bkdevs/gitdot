import { z } from "zod";
import { WebhookResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateWebhookRequest = z.object({
  url: z.string(),
  secret: z.string(),
  events: z.array(z.string()),
});
export type CreateWebhookRequest = z.infer<typeof CreateWebhookRequest>;

export const CreateWebhookResponse = WebhookResource;
export type CreateWebhookResponse = z.infer<typeof CreateWebhookResponse>;

export const CreateWebhook = {
  path: "/repository/{owner}/{repo}/webhook",
  method: "POST",
  request: CreateWebhookRequest,
  response: CreateWebhookResponse,
} as const satisfies Endpoint;
export type CreateWebhook = typeof CreateWebhook;
