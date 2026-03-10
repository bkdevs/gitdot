"use server";

import { createSpan } from "@/dal";

export async function createSpanAction(
  url: string,
  startTime: number,
  endTime: number,
): Promise<void> {
  await createSpan({ url, start_time: startTime, end_time: endTime });
}
