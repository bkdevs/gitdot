import "server-only";

import {
  type AuthorizeDeviceRequest,
  SlackAccountResource,
} from "gitdot-api";
import { GITDOT_AUTH_SERVER_URL } from "@/lib/auth";
import { authPost, handleResponse } from "./util";

export async function authorizeDevice(
  request: AuthorizeDeviceRequest,
): Promise<boolean> {
  const response = await authPost(
    `${GITDOT_AUTH_SERVER_URL}/auth/device/authorize`,
    request,
  );

  return response.ok;
}

export async function linkSlackAccount(
  state: string,
): Promise<SlackAccountResource | null> {
  const response = await authPost(
    `${GITDOT_AUTH_SERVER_URL}/auth/slack/link`,
    { state },
  );

  return await handleResponse(response, SlackAccountResource);
}
