import "server-only";

import type { AuthorizeDeviceRequest } from "gitdot-api";
import { GITDOT_AUTH_SERVER_URL } from "@/lib/auth";
import { authPost } from "./util";

export async function authorizeDevice(
  request: AuthorizeDeviceRequest,
): Promise<boolean> {
  const response = await authPost(
    `${GITDOT_AUTH_SERVER_URL}/auth/device/authorize`,
    request,
  );

  return response.ok;
}
