import { GITDOT_SERVER_URL } from "@/dal/util";
import { getSession } from "@/lib/supabase";

export async function GET(
  _request: Request,
  { params }: { params: Promise<{ id: string }> },
) {
  const { id } = await params;
  const session = await getSession();

  const upstream = await fetch(
    `${GITDOT_SERVER_URL}/ci/task/${encodeURIComponent(id)}/logs`,
    {
      headers: {
        Accept: "text/event-stream",
        ...(session && { Authorization: `Bearer ${session.access_token}` }),
      },
    },
  );

  return new Response(upstream.body, {
    status: upstream.status,
    headers: {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      Connection: "keep-alive",
    },
  });
}
