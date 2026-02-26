import { z } from "zod";
import { UserResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateCurrentUserRequest = z.object({
  name: z.string(),
});
export type UpdateCurrentUserRequest = z.infer<typeof UpdateCurrentUserRequest>;

export const UpdateCurrentUserResponse = UserResource;
export type UpdateCurrentUserResponse = z.infer<
  typeof UpdateCurrentUserResponse
>;

export const UpdateCurrentUser = {
  path: "/user",
  method: "PATCH",
  request: UpdateCurrentUserRequest,
  response: UpdateCurrentUserResponse,
} satisfies Endpoint;
export type UpdateCurrentUser = typeof UpdateCurrentUser;
