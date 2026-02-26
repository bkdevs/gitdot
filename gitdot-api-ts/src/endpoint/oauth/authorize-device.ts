import { z } from "zod";

export const AuthorizeDeviceRequest = z.object({
  user_code: z.string(),
});
export type AuthorizeDeviceRequest = z.infer<typeof AuthorizeDeviceRequest>;

export const AuthorizeDevice = {
  path: "/oauth/authorize",
  method: "POST",
  request: AuthorizeDeviceRequest,
  response: z.void(),
} as const;
export type AuthorizeDevice = typeof AuthorizeDevice;
