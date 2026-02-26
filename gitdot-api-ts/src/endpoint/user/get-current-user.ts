import { z } from "zod";

import { UserResource } from "../../resource";

export const GetCurrentUserRequest = z.object({});
export type GetCurrentUserRequest = z.infer<typeof GetCurrentUserRequest>;

export const GetCurrentUser = {
  path: "/user",
  method: "GET",
  request: GetCurrentUserRequest,
  response: UserResource,
} as const;
export type GetCurrentUser = typeof GetCurrentUser;
