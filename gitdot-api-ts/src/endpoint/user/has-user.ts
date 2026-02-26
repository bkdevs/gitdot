import { z } from "zod";

export const HasUserRequest = z.object({
  user_name: z.string(),
});
export type HasUserRequest = z.infer<typeof HasUserRequest>;

export const HasUser = {
  path: "/user/{user_name}",
  method: "HEAD",
  request: HasUserRequest,
  response: z.void(),
} as const;
export type HasUser = typeof HasUser;
