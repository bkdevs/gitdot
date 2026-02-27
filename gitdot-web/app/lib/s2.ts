import "server-only";

const S2_SERVER_URL =
  process.env.NEXT_PUBLIC_S2_SERVER_URL ?? "http://localhost:8081";

export interface S2Record {
  seq_num: number;
  body: string;
}

export interface GetTaskLogsOptions {
  tailOffset?: number; // default 100
  count?: number; // default 100
  clamp?: boolean; // default true
}

export async function getTaskLogs(
  owner: string,
  repo: string,
  taskId: string,
  options: GetTaskLogsOptions = {},
): Promise<S2Record[]> {
  const { tailOffset = 100, count = 100, clamp = true } = options;
  const url = new URL(
    `/v1/streams/${encodeURIComponent(`task/${taskId}`)}/records`,
    S2_SERVER_URL,
  );
  url.searchParams.set("tail_offset", String(tailOffset));
  url.searchParams.set("count", String(count));
  url.searchParams.set("clamp", String(clamp));

  const response = await fetch(url.toString(), {
    headers: {
      "s2-basin": `${owner}-${repo}`,
    },
  });

  if (!response.ok) {
    throw new Error(
      `S2 request failed: ${response.status} ${response.statusText}`,
    );
  }

  return response.json() as Promise<S2Record[]>;
}
