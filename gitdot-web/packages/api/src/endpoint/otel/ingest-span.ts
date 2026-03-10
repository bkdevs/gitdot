import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const IngestSpanRequest = z.object({
  url: z.string(),
  start_time: z.number(),
  end_time: z.number(),
});
export type IngestSpanRequest = z.infer<typeof IngestSpanRequest>;

export const IngestSpanResponse = z.void();
export type IngestSpanResponse = z.infer<typeof IngestSpanResponse>;

export const IngestSpan = {
  path: "/otel/spans",
  method: "POST",
  request: IngestSpanRequest,
  response: IngestSpanResponse,
} as const satisfies Endpoint;
export type IngestSpan = typeof IngestSpan;
