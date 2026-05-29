import { renderCommitDiff } from "gitdot-dal/server";
import type { NextRequest } from "next/server";

export async function GET(request: NextRequest) {
  const { searchParams } = request.nextUrl;
  const owner = searchParams.get("owner") ?? "";
  const repo = searchParams.get("repo") ?? "";
  const sha = searchParams.get("sha") ?? "";
  return Response.json(await renderCommitDiff(owner, repo, sha));
}
