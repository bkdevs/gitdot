import { z } from "zod";

import { RepositoryPreviewResource } from "../../resource";

export const GetRepositoryPreviewRequest = z.object({
  ref_name: z.string(),
  preview_lines: z.number().int().optional(),
});
export type GetRepositoryPreviewRequest = z.infer<
  typeof GetRepositoryPreviewRequest
>;

export const GetRepositoryPreview = {
  path: "/repository/{owner}/{repo}/preview",
  method: "GET",
  request: GetRepositoryPreviewRequest,
  response: RepositoryPreviewResource,
} as const;
export type GetRepositoryPreview = typeof GetRepositoryPreview;
