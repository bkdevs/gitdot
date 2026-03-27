import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const DeleteWebhook = {
  path: "/repository/{owner}/{repo}/webhook/{webhook_id}",
  method: "DELETE",
  request: z.object({}),
  response: z.object({}),
} as const satisfies Endpoint;
export type DeleteWebhook = typeof DeleteWebhook;
