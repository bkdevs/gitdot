"use client";

const S2_SERVER_URL =
  process.env.NEXT_PUBLIC_S2_SERVER_URL ?? "http://localhost:8081";

export interface S2Record {
  seq_num: number;
  body: string;
}

export interface TailTaskLogsOptions {
  lastEventId?: string; // for resumption: "{seq_num},{count},{bytes}"
}

export function tailTaskLogs(
  owner: string,
  repo: string,
  taskId: string,
  onBatch: (records: S2Record[]) => void,
  options: TailTaskLogsOptions = {},
): AbortController {
  const controller = new AbortController();

  const url = new URL(
    `/v1/streams/${encodeURIComponent(`task/${taskId}`)}/records`,
    S2_SERVER_URL,
  );
  url.searchParams.set("tail_offset", "0");

  const headers: Record<string, string> = {
    "s2-basin": `${owner}-${repo}`,
    Accept: "text/event-stream",
  };
  if (options.lastEventId) {
    headers["Last-Event-Id"] = options.lastEventId;
  }

  (async () => {
    try {
      const response = await fetch(url.toString(), {
        headers,
        signal: controller.signal,
      });

      if (!response.ok || !response.body) {
        console.error(
          `S2 stream failed: ${response.status} ${response.statusText}`,
        );
        return;
      }

      const reader = response.body.getReader();
      const decoder = new TextDecoder();
      let buffer = "";

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        buffer += decoder.decode(value, { stream: true });

        // SSE events are separated by double newlines
        const events = buffer.split("\n\n");
        buffer = events.pop() ?? "";

        for (const event of events) {
          if (!event.trim()) continue;

          let eventType = "message";
          let data = "";

          for (const line of event.split("\n")) {
            if (line.startsWith("event:")) {
              eventType = line.slice("event:".length).trim();
            } else if (line.startsWith("data:")) {
              data = line.slice("data:".length).trim();
            }
          }

          if (eventType === "batch") {
            try {
              const records = JSON.parse(data) as S2Record[];
              onBatch(records);
            } catch (err) {
              console.error("Failed to parse S2 batch data:", err);
            }
          } else if (eventType === "error") {
            console.error("S2 stream error event:", data);
          }
          // ping: no-op
        }
      }
    } catch (err) {
      if (err instanceof Error && err.name === "AbortError") {
        // Clean abort, no action needed
        return;
      }
      console.error("S2 tail stream error:", err);
    }
  })();

  return controller;
}
