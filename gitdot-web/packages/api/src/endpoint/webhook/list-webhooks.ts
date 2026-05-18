import { z } from "zod";
import { page, WebhookResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListWebhooksRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListWebhooksRequest = z.infer<typeof ListWebhooksRequest>;

export const ListWebhooksResponse = page(WebhookResource);
export type ListWebhooksResponse = z.infer<typeof ListWebhooksResponse>;

export const ListWebhooks = {
  path: "/repository/{owner}/{repo}/webhooks",
  method: "GET",
  request: ListWebhooksRequest,
  response: ListWebhooksResponse,
} as const satisfies Endpoint;
export type ListWebhooks = typeof ListWebhooks;
