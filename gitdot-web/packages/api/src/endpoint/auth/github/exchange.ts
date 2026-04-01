import { z } from "zod";
import { AuthTokensResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const ExchangeGitHubCodeRequest = z.object({
  code: z.string(),
  state: z.string(),
});
export type ExchangeGitHubCodeRequest = z.infer<
  typeof ExchangeGitHubCodeRequest
>;

export const ExchangeGitHubCodeResponse = AuthTokensResource;
export type ExchangeGitHubCodeResponse = z.infer<
  typeof ExchangeGitHubCodeResponse
>;

export const ExchangeGitHubCode = {
  path: "/auth/github/exchange",
  method: "POST",
  request: ExchangeGitHubCodeRequest,
  response: ExchangeGitHubCodeResponse,
} as const satisfies Endpoint;

export type ExchangeGitHubCode = typeof ExchangeGitHubCode;
