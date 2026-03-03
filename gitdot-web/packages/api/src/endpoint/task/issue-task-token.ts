import { z } from "zod";
import { TaskTokenResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const IssueTaskTokenRequest = z.object({});
export type IssueTaskTokenRequest = z.infer<typeof IssueTaskTokenRequest>;

export const IssueTaskTokenResponse = TaskTokenResource;
export type IssueTaskTokenResponse = z.infer<typeof IssueTaskTokenResponse>;

export const IssueTaskToken = {
  path: "/ci/task/{id}/token",
  method: "POST",
  request: IssueTaskTokenRequest,
  response: IssueTaskTokenResponse,
} as const satisfies Endpoint;
export type IssueTaskToken = typeof IssueTaskToken;
