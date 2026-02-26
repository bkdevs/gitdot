import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const AuthorizeDeviceRequest = z.object({
  user_code: z.string(),
});
export type AuthorizeDeviceRequest = z.infer<typeof AuthorizeDeviceRequest>;

export const AuthorizeDeviceResponse = z.void();
export type AuthorizeDeviceResponse = z.infer<typeof AuthorizeDeviceResponse>;

export const AuthorizeDevice = {
  path: "/oauth/authorize",
  method: "POST",
  request: AuthorizeDeviceRequest,
  response: AuthorizeDeviceResponse,
} satisfies Endpoint;

export type AuthorizeDevice = typeof AuthorizeDevice;
