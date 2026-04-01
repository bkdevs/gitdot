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
