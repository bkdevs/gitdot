"use server";

import { createSpan } from "@/dal";

const otelEnabled = process.env.OTEL_ENABLED === "1";

export async function createSpanAction(
  url: string,
  startTime: number,
  endTime: number,
): Promise<void> {
  otelEnabled && await createSpan({ url, start_time: startTime, end_time: endTime });
}
