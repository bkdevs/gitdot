import { z } from "zod";
import { TokenResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const PollTokenRequest = z.object({
  device_code: z.string(),
  client_id: z.string(),
});
export type PollTokenRequest = z.infer<typeof PollTokenRequest>;

export const PollTokenResponse = TokenResource;
export type PollTokenResponse = z.infer<typeof PollTokenResponse>;

export const PollToken = {
  path: "/oauth/token",
  method: "POST",
  request: PollTokenRequest,
  response: PollTokenResponse,
} satisfies Endpoint;
export type PollToken = typeof PollToken;
