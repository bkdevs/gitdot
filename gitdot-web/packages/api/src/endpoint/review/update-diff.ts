import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateDiffRequest = z.object({
  title: z.string().optional(),
  description: z.string().optional(),
});
export type UpdateDiffRequest = z.infer<typeof UpdateDiffRequest>;

export const UpdateDiffResponse = ReviewResource;
export type UpdateDiffResponse = z.infer<typeof UpdateDiffResponse>;

export const UpdateDiff = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}",
  method: "PATCH",
  request: UpdateDiffRequest,
  response: UpdateDiffResponse,
} as const satisfies Endpoint;
export type UpdateDiff = typeof UpdateDiff;
