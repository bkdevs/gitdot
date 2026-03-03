export const S2_SERVER_URL =
  process.env.NEXT_PUBLIC_S2_SERVER_URL ?? "http://localhost:8081";

export interface S2Record {
  seq_num: number;
  timestamp: number;
  body: string;
  headers: string[][];
}
