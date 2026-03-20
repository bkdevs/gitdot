import { z } from "zod";

import { RepositorySettingsResource } from "../../resource";
import { CommitFilterResource } from "../../resource/settings";
import type { Endpoint } from "../endpoint";

export const UpdateRepositorySettingsRequest = z.object({
  commit_filters: z.array(CommitFilterResource).optional(),
});
export type UpdateRepositorySettingsRequest = z.infer<
  typeof UpdateRepositorySettingsRequest
>;

export const UpdateRepositorySettingsResponse = RepositorySettingsResource;
export type UpdateRepositorySettingsResponse = z.infer<
  typeof UpdateRepositorySettingsResponse
>;

export const UpdateRepositorySettings = {
  path: "/repository/{owner}/{repo}/settings",
  method: "PATCH",
  request: UpdateRepositorySettingsRequest,
  response: UpdateRepositorySettingsResponse,
} as const satisfies Endpoint;
export type UpdateRepositorySettings = typeof UpdateRepositorySettings;
