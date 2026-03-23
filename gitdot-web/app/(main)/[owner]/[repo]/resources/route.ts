import { promisify } from "node:util";
import { gzip } from "node:zlib";
import { GetRepositoryResourcesRequest } from "gitdot-api";
import type { NextRequest } from "next/server";
import { getRepositoryResources } from "@/dal";

const gzipAsync = promisify(gzip);

// TODO: make the client call gitdot-api directly rather than go through the route
// the main reason we're doing so is authentication, as though we _can_ forward SameSite cookies with
// Origin set to Lax (i.e., gitdot.io -> api.gitdot.io), we do not have the logic to decode the Supabase JWT key on our backend
//
// TODO: should block users from hitting this too, only the worker ever should as it is indeed expensive.
export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ owner: string; repo: string }> },
) {
  const { owner, repo } = await params;
  const { searchParams } = request.nextUrl;

  const parsed = GetRepositoryResourcesRequest.safeParse({
    last_commit: searchParams.get("last_commit") ?? undefined,
    last_updated: searchParams.get("last_updated") ?? undefined,
  });
  const result = await getRepositoryResources(
    owner,
    repo,
    parsed.success ? parsed.data : {},
  );

  if (!result) return new Response(null, { status: 404 });

  const compressed = await gzipAsync(JSON.stringify(result));
  return new Response(compressed, {
    headers: {
      "Content-Type": "application/json",
      "Content-Encoding": "gzip",
    },
  });
}
