import { z } from "zod";

import { RepositoryPermissionResource } from "../../resource";

export const GetPermissionRequest = z.object({});
export type GetPermissionRequest = z.infer<typeof GetPermissionRequest>;

export const GetPermission = {
  path: "/repository/{owner}/{repo}/permission",
  method: "GET",
  request: GetPermissionRequest,
  response: RepositoryPermissionResource,
} as const;
export type GetPermission = typeof GetPermission;
