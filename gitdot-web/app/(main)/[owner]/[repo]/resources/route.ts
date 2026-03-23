import type { NextRequest } from "next/server";
import { NextResponse } from "next/server";
import { getRepositoryResources } from "@/dal";

export async function GET(
  _request: NextRequest,
  { params }: { params: Promise<{ owner: string; repo: string }> },
) {
  const { owner, repo } = await params;
  const result = await getRepositoryResources(owner, repo);

  if (!result) return new NextResponse(null, { status: 404 });
  return NextResponse.json(result);
}
