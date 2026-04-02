import { z } from "zod";

export const AuthTokensResource = z.object({
  access_token: z.string(),
  refresh_token: z.string(),
  access_token_expires_in: z.number(),
  refresh_token_expires_in: z.number(),
  is_new: z.boolean(),
});
export type AuthTokensResource = z.infer<typeof AuthTokensResource>;

export const GitHubAuthRedirectResource = z.object({
  authorize_url: z.string(),
  state: z.string(),
});
export type GitHubAuthRedirectResource = z.infer<
  typeof GitHubAuthRedirectResource
>;

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
