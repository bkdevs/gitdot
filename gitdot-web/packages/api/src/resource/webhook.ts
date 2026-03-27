import { z } from "zod";

export const WebhookResource = z.object({
  id: z.uuid(),
  repository_id: z.uuid(),
  url: z.string(),
  secret: z.string(),
  events: z.array(z.string()),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});
export type WebhookResource = z.infer<typeof WebhookResource>;
