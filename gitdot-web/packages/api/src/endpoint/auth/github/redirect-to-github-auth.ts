import { z } from "zod";
import { GitHubAuthRedirectResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const RedirectToGitHubAuthResponse = GitHubAuthRedirectResource;
export type RedirectToGitHubAuthResponse = z.infer<
  typeof RedirectToGitHubAuthResponse
>;

export const RedirectToGitHubAuth = {
  path: "/auth/github/redirect",
  method: "GET",
  request: z.void(),
  response: RedirectToGitHubAuthResponse,
} as const satisfies Endpoint;

export type RedirectToGitHubAuth = typeof RedirectToGitHubAuth;
