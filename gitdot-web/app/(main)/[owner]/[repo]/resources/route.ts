import { promisify } from "node:util";
import { gzip } from "node:zlib";
import type { NextRequest } from "next/server";
import { getRepositoryResources } from "@/dal";

const gzipAsync = promisify(gzip);

export async function GET(
  _request: NextRequest,
  { params }: { params: Promise<{ owner: string; repo: string }> },
) {
  const { owner, repo } = await params;
  const result = await getRepositoryResources(owner, repo);

  if (!result) return new Response(null, { status: 404 });

  const compressed = await gzipAsync(JSON.stringify(result));
  return new Response(compressed, {
    headers: {
      "Content-Type": "application/json",
      "Content-Encoding": "gzip",
    },
  });
}
