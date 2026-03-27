import { z } from "zod";
import { WebhookResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListWebhooksResponse = z.array(WebhookResource);
export type ListWebhooksResponse = z.infer<typeof ListWebhooksResponse>;

export const ListWebhooks = {
  path: "/repository/{owner}/{repo}/webhooks",
  method: "GET",
  request: z.object({}),
  response: ListWebhooksResponse,
} as const satisfies Endpoint;
export type ListWebhooks = typeof ListWebhooks;
