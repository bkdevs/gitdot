import { z } from "zod";

import { UserResource } from "../../resource";

export const UpdateCurrentUserRequest = z.object({
  name: z.string(),
});
export type UpdateCurrentUserRequest = z.infer<typeof UpdateCurrentUserRequest>;

export const UpdateCurrentUser = {
  path: "/user",
  method: "PATCH",
  request: UpdateCurrentUserRequest,
  response: UserResource,
} as const;
export type UpdateCurrentUser = typeof UpdateCurrentUser;
