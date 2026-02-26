import { z } from "zod";

import { TokenResource } from "../../resource";

export const PollTokenRequest = z.object({
  device_code: z.string(),
  client_id: z.string(),
});
export type PollTokenRequest = z.infer<typeof PollTokenRequest>;

export const PollToken = {
  path: "/oauth/token",
  method: "POST",
  request: PollTokenRequest,
  response: TokenResource,
} as const;
export type PollToken = typeof PollToken;
