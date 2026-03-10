import "server-only";

import { getVercelOidcToken } from "@vercel/oidc";
import type { CreateSpanRequest } from "gitdot-api";
import { GITDOT_SERVER_URL, handleEmptyResponse } from "./util";

export async function createSpan(request: CreateSpanRequest): Promise<void> {
  const token = await getVercelOidcToken();

  const response = await fetch(`${GITDOT_SERVER_URL}/otel/spans`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(request),
  });

  await handleEmptyResponse(response);
}
