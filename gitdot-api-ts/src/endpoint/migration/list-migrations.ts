import { z } from "zod";

import { MigrationResource } from "../../resource";

export const ListMigrationsRequest = z.object({});
export type ListMigrationsRequest = z.infer<typeof ListMigrationsRequest>;

export const ListMigrations = {
  path: "/migrations",
  method: "GET",
  request: ListMigrationsRequest,
  response: z.array(MigrationResource),
} as const;
export type ListMigrations = typeof ListMigrations;
