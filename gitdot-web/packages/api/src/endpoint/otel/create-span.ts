import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const CreateSpanRequest = z.object({
  url: z.string(),
  start_time: z.number(),
  end_time: z.number(),
});
export type CreateSpanRequest = z.infer<typeof CreateSpanRequest>;

export const CreateSpanResponse = z.void();
export type CreateSpanResponse = z.infer<typeof CreateSpanResponse>;

export const CreateSpan = {
  path: "/otel/spans",
  method: "POST",
  request: CreateSpanRequest,
  response: CreateSpanResponse,
} as const satisfies Endpoint;
export type CreateSpan = typeof CreateSpan;
