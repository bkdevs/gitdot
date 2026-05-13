import { z } from "zod";
import { RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListUserStarsRequest = z.object({});
export type ListUserStarsRequest = z.infer<typeof ListUserStarsRequest>;

export const ListUserStarsResponse = z.array(RepositoryResource);
export type ListUserStarsResponse = z.infer<typeof ListUserStarsResponse>;

export const ListUserStars = {
  path: "/user/{user_name}/stars",
  method: "GET",
  request: ListUserStarsRequest,
  response: ListUserStarsResponse,
} as const satisfies Endpoint;
export type ListUserStars = typeof ListUserStars;
