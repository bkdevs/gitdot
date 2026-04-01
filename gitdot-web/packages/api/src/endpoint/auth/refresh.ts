import { z } from "zod";
import { AuthTokensResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const RefreshSessionRequest = z.object({
  refresh_token: z.string(),
});
export type RefreshSessionRequest = z.infer<typeof RefreshSessionRequest>;

export const RefreshSessionResponse = AuthTokensResource;
export type RefreshSessionResponse = z.infer<typeof RefreshSessionResponse>;

export const RefreshSession = {
  path: "/auth/refresh",
  method: "POST",
  request: RefreshSessionRequest,
  response: RefreshSessionResponse,
} as const satisfies Endpoint;

export type RefreshSession = typeof RefreshSession;
