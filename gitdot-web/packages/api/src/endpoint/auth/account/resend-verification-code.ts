import { z } from "zod";
import type { Endpoint } from "../../endpoint";

export const ResendVerificationCodeRequest = z.object({
  email: z.string(),
});
export type ResendVerificationCodeRequest = z.infer<
  typeof ResendVerificationCodeRequest
>;

export const ResendVerificationCode = {
  path: "/auth/account/resend-code",
  method: "POST",
  request: ResendVerificationCodeRequest,
  response: z.void(),
} as const satisfies Endpoint;
export type ResendVerificationCode = typeof ResendVerificationCode;
