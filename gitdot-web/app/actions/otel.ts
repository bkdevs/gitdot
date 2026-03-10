"use server";

import { ingestSpan } from "@/dal";

export async function ingestSpanAction(
  url: string,
  startTime: number,
  endTime: number,
): Promise<void> {
  await ingestSpan({ url, start_time: startTime, end_time: endTime });
}
