import { z } from "zod";
import { RepositoryPreviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryPreviewRequest = z.object({
  ref_name: z.string(),
  preview_lines: z.number().int().optional(),
});
export type GetRepositoryPreviewRequest = z.infer<
  typeof GetRepositoryPreviewRequest
>;

export const GetRepositoryPreviewResponse = RepositoryPreviewResource;
export type GetRepositoryPreviewResponse = z.infer<
  typeof GetRepositoryPreviewResponse
>;

export const GetRepositoryPreview = {
  path: "/repository/{owner}/{repo}/preview",
  method: "GET",
  request: GetRepositoryPreviewRequest,
  response: GetRepositoryPreviewResponse,
} satisfies Endpoint;
export type GetRepositoryPreview = typeof GetRepositoryPreview;
