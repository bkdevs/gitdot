import { z } from "zod";

export const DeviceCodeResource = z.object({
  device_code: z.string(),
  user_code: z.string(),
  verification_uri: z.string(),
  expires_in: z.number(),
  interval: z.number(),
});
export type DeviceCodeResource = z.infer<typeof DeviceCodeResource>;

export const TokenResource = z.object({
  access_token: z.string(),
  user_name: z.string(),
  user_email: z.string(),
});
export type TokenResource = z.infer<typeof TokenResource>;
