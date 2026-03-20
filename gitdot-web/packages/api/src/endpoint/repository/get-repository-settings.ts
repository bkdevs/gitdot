import { z } from "zod";

import { RepositorySettingsResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositorySettingsRequest = z.object({});
export type GetRepositorySettingsRequest = z.infer<
  typeof GetRepositorySettingsRequest
>;

export const GetRepositorySettingsResponse = RepositorySettingsResource;
export type GetRepositorySettingsResponse = z.infer<
  typeof GetRepositorySettingsResponse
>;

export const GetRepositorySettings = {
  path: "/repository/{owner}/{repo}/settings",
  method: "GET",
  request: GetRepositorySettingsRequest,
  response: GetRepositorySettingsResponse,
} as const satisfies Endpoint;
export type GetRepositorySettings = typeof GetRepositorySettings;
