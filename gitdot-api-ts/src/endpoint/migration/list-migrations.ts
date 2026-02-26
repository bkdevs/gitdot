import { z } from "zod";
import { MigrationResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListMigrationsRequest = z.object({});
export type ListMigrationsRequest = z.infer<typeof ListMigrationsRequest>;

export const ListMigrationsResponse = z.array(MigrationResource);
export type ListMigrationsResponse = z.infer<typeof ListMigrationsResponse>;

export const ListMigrations = {
  path: "/migrations",
  method: "GET",
  request: ListMigrationsRequest,
  response: ListMigrationsResponse,
} satisfies Endpoint;
export type ListMigrations = typeof ListMigrations;
