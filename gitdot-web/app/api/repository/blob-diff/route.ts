import { renderBlobDiffs } from "gitdot-dal/server";
import type { NextRequest } from "next/server";

export async function GET(request: NextRequest) {
  const { searchParams } = request.nextUrl;
  const owner = searchParams.get("owner") ?? "";
  const repo = searchParams.get("repo") ?? "";
  const path = searchParams.get("path") ?? "";
  const commitShas = searchParams.getAll("sha");
  return Response.json(await renderBlobDiffs(owner, repo, commitShas, path));
}
