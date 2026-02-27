import { z } from "zod";
import { RepositoryPermissionResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetPermissionRequest = z.object({});
export type GetPermissionRequest = z.infer<typeof GetPermissionRequest>;

export const GetPermissionResponse = RepositoryPermissionResource;
export type GetPermissionResponse = z.infer<typeof GetPermissionResponse>;

export const GetPermission = {
  path: "/repository/{owner}/{repo}/permission",
  method: "GET",
  request: GetPermissionRequest,
  response: GetPermissionResponse,
} as const satisfies Endpoint;
export type GetPermission = typeof GetPermission;
