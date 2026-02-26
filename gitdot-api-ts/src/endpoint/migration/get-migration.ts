import { z } from "zod";

import { MigrationResource } from "../../resource";

export const GetMigrationRequest = z.object({});
export type GetMigrationRequest = z.infer<typeof GetMigrationRequest>;

export const GetMigration = {
  path: "/migration/{number}",
  method: "GET",
  request: GetMigrationRequest,
  response: MigrationResource,
} as const;
export type GetMigration = typeof GetMigration;
