import type { z } from "zod";

export interface Endpoint {
  path: string;
  method: "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD";
  request: z.ZodType;
  response: z.ZodType;
}
