import { z } from "zod";
import { MigrationResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetMigrationRequest = z.object({});
export type GetMigrationRequest = z.infer<typeof GetMigrationRequest>;

export const GetMigrationResponse = MigrationResource;
export type GetMigrationResponse = z.infer<typeof GetMigrationResponse>;

export const GetMigration = {
  path: "/migration/{number}",
  method: "GET",
  request: GetMigrationRequest,
  response: GetMigrationResponse,
} satisfies Endpoint;
export type GetMigration = typeof GetMigration;
