import { z } from "zod";
import { WebhookResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetWebhookResponse = WebhookResource;
export type GetWebhookResponse = z.infer<typeof GetWebhookResponse>;

export const GetWebhook = {
  path: "/repository/{owner}/{repo}/webhook/{webhook_id}",
  method: "GET",
  request: z.object({}),
  response: GetWebhookResponse,
} as const satisfies Endpoint;
export type GetWebhook = typeof GetWebhook;
