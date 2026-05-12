import { z } from "zod";
import { CurrentUserResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetCurrentUserRequest = z.object({});
export type GetCurrentUserRequest = z.infer<typeof GetCurrentUserRequest>;

export const GetCurrentUserResponse = CurrentUserResource;
export type GetCurrentUserResponse = z.infer<typeof GetCurrentUserResponse>;

export const GetCurrentUser = {
  path: "/user",
  method: "GET",
  request: GetCurrentUserRequest,
  response: GetCurrentUserResponse,
} as const satisfies Endpoint;
export type GetCurrentUser = typeof GetCurrentUser;
