import { z } from "zod";

import { UserResource } from "../../resource";

export const GetUserRequest = z.object({});
export type GetUserRequest = z.infer<typeof GetUserRequest>;

export const GetUser = {
  path: "/user/{user_name}",
  method: "GET",
  request: GetUserRequest,
  response: UserResource,
} as const;
export type GetUser = typeof GetUser;
