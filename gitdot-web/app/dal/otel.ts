import "server-only";

import { getVercelOidcToken } from "@vercel/oidc";
import type { IngestSpanRequest } from "gitdot-api";
import { GITDOT_SERVER_URL, handleEmptyResponse } from "./util";

export async function ingestSpan(request: IngestSpanRequest): Promise<void> {
  const token = await getVercelOidcToken();
  console.log(token);

  const response = await fetch(`${GITDOT_SERVER_URL}/otel/spans`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });

  await handleEmptyResponse(response);
}
