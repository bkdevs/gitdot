import type { z } from "zod";

export interface Endpoint<
  TReq extends z.ZodTypeAny,
  TRes extends z.ZodTypeAny,
> {
  path: string;
  method: "GET" | "POST" | "PATCH" | "DELETE" | "HEAD" | "PUT";
  request: TReq;
  response: TRes;
}
