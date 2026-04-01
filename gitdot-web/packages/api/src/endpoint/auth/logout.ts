import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const LogoutRequest = z.object({
  refresh_token: z.string(),
});
export type LogoutRequest = z.infer<typeof LogoutRequest>;

export const Logout = {
  path: "/auth/logout",
  method: "POST",
  request: LogoutRequest,
  response: z.void(),
} as const satisfies Endpoint;

export type Logout = typeof Logout;
