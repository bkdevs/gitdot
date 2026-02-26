import { z } from "zod";
import { UserResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetCurrentUserRequest = z.object({});
export type GetCurrentUserRequest = z.infer<typeof GetCurrentUserRequest>;

export const GetCurrentUserResponse = UserResource;
export type GetCurrentUserResponse = z.infer<typeof GetCurrentUserResponse>;

export const GetCurrentUser = {
  path: "/user",
  method: "GET",
  request: GetCurrentUserRequest,
  response: GetCurrentUserResponse,
} satisfies Endpoint;
export type GetCurrentUser = typeof GetCurrentUser;
