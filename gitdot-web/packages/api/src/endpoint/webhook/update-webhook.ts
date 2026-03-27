import { z } from "zod";
import { WebhookResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateWebhookRequest = z.object({
  url: z.string().optional(),
  secret: z.string().optional(),
  events: z.array(z.string()).optional(),
});
export type UpdateWebhookRequest = z.infer<typeof UpdateWebhookRequest>;

export const UpdateWebhookResponse = WebhookResource;
export type UpdateWebhookResponse = z.infer<typeof UpdateWebhookResponse>;

export const UpdateWebhook = {
  path: "/repository/{owner}/{repo}/webhook/{webhook_id}",
  method: "PATCH",
  request: UpdateWebhookRequest,
  response: UpdateWebhookResponse,
} as const satisfies Endpoint;
export type UpdateWebhook = typeof UpdateWebhook;
