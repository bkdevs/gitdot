import "server-only";

import {
  type AuthorizeDeviceRequest,
  DeviceCodeResource,
  type PollTokenRequest,
  TokenResource,
} from "gitdot-api";
import { toQueryString } from "@/util";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

type GetDeviceCodeQuery = { client_id: string };

export async function getDeviceCode(
  query: GetDeviceCodeQuery,
): Promise<DeviceCodeResource | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/oauth/device?${queryString}`,
    { method: "POST" },
  );

  return await handleResponse(response, DeviceCodeResource);
}

export async function pollToken(
  request: PollTokenRequest,
): Promise<TokenResource | null> {
  const response = await authPost(`${GITDOT_SERVER_URL}/oauth/token`, request);

  return await handleResponse(response, TokenResource);
}

export async function authorizeDevice(
  request: AuthorizeDeviceRequest,
): Promise<boolean> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/oauth/authorize`,
    request,
  );

  return response.ok;
}
