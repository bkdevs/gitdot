import { z } from "zod";
import { RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ConvertReadonlyRepositoryRequest = z.object({});
export type ConvertReadonlyRepositoryRequest = z.infer<
  typeof ConvertReadonlyRepositoryRequest
>;

export const ConvertReadonlyRepositoryResponse = RepositoryResource;
export type ConvertReadonlyRepositoryResponse = z.infer<
  typeof ConvertReadonlyRepositoryResponse
>;

export const ConvertReadonlyRepository = {
  path: "/repository/{owner}/{repo}/convert-readonly",
  method: "POST",
  request: ConvertReadonlyRepositoryRequest,
  response: ConvertReadonlyRepositoryResponse,
} as const satisfies Endpoint;
export type ConvertReadonlyRepository = typeof ConvertReadonlyRepository;
