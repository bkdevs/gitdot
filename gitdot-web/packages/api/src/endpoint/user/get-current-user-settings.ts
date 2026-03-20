import { z } from "zod";
import { UserSettingsResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetCurrentUserSettingsRequest = z.object({});
export type GetCurrentUserSettingsRequest = z.infer<
  typeof GetCurrentUserSettingsRequest
>;

export const GetCurrentUserSettingsResponse = UserSettingsResource;
export type GetCurrentUserSettingsResponse = z.infer<
  typeof GetCurrentUserSettingsResponse
>;

export const GetCurrentUserSettings = {
  path: "/user/settings",
  method: "GET",
  request: GetCurrentUserSettingsRequest,
  response: GetCurrentUserSettingsResponse,
} as const satisfies Endpoint;
export type GetCurrentUserSettings = typeof GetCurrentUserSettings;
