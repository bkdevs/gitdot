import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const HasUserRequest = z.object({
  user_name: z.string(),
});
export type HasUserRequest = z.infer<typeof HasUserRequest>;

export const HasUserResponse = z.void();
export type HasUserResponse = z.infer<typeof HasUserResponse>;

export const HasUser = {
  path: "/user/{user_name}",
  method: "HEAD",
  request: HasUserRequest,
  response: HasUserResponse,
} as const satisfies Endpoint;
export type HasUser = typeof HasUser;
