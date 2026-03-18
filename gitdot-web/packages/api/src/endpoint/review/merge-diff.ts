import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const MergeDiffRequest = z.object({});
export type MergeDiffRequest = z.infer<typeof MergeDiffRequest>;

export const MergeDiffResponse = ReviewResource;
export type MergeDiffResponse = z.infer<typeof MergeDiffResponse>;

export const MergeDiff = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}/merge",
  method: "POST",
  request: MergeDiffRequest,
  response: MergeDiffResponse,
} as const satisfies Endpoint;
export type MergeDiff = typeof MergeDiff;
