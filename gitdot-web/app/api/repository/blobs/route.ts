import { getRepositoryBlobs } from "gitdot-client";
import type { NextRequest } from "next/server";

export async function GET(request: NextRequest) {
  const { searchParams } = request.nextUrl;
  const owner = searchParams.get("owner") ?? "";
  const repo = searchParams.get("repo") ?? "";
  const path = searchParams.get("path") ?? "";
  const refs = searchParams.getAll("ref");
  const result = await getRepositoryBlobs(owner, repo, { refs, paths: [path] });
  return Response.json(result?.blobs ?? []);
}
