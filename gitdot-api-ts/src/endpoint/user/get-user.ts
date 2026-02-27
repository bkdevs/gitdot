import { z } from "zod";
import { UserResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetUserRequest = z.object({});
export type GetUserRequest = z.infer<typeof GetUserRequest>;

export const GetUserResponse = UserResource;
export type GetUserResponse = z.infer<typeof GetUserResponse>;

export const GetUser = {
  path: "/user/{user_name}",
  method: "GET",
  request: GetUserRequest,
  response: GetUserResponse,
} as const satisfies Endpoint;
export type GetUser = typeof GetUser;
