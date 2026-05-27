import { z } from "zod";
import { UserEmailResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const VerifyUserEmailRequest = z.object({
  email: z.string(),
  code: z.string(),
});
export type VerifyUserEmailRequest = z.infer<typeof VerifyUserEmailRequest>;

export const VerifyUserEmailResponse = UserEmailResource;
export type VerifyUserEmailResponse = z.infer<typeof VerifyUserEmailResponse>;

export const VerifyUserEmail = {
  path: "/auth/email/verify-email",
  method: "POST",
  request: VerifyUserEmailRequest,
  response: VerifyUserEmailResponse,
} as const satisfies Endpoint;
export type VerifyUserEmail = typeof VerifyUserEmail;
