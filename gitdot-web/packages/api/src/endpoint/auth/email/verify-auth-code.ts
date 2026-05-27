import { z } from "zod";
import { AuthTokensResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const VerifyAuthCodeRequest = z.object({
  code: z.string(),
});
export type VerifyAuthCodeRequest = z.infer<typeof VerifyAuthCodeRequest>;

export const VerifyAuthCodeResponse = AuthTokensResource;
export type VerifyAuthCodeResponse = z.infer<typeof VerifyAuthCodeResponse>;

export const VerifyAuthCode = {
  path: "/auth/email/verify",
  method: "POST",
  request: VerifyAuthCodeRequest,
  response: VerifyAuthCodeResponse,
} as const satisfies Endpoint;

export type VerifyAuthCode = typeof VerifyAuthCode;
