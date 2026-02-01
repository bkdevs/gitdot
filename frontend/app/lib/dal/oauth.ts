import "server-only";

import { toQueryString } from "@/util";
import {
  type AuthorizeDeviceRequest,
  type DeviceCodeResponse,
  DeviceCodeResponseSchema,
  type GetDeviceCodeQuery,
  type PollTokenRequest,
  type TokenResponse,
  TokenResponseSchema,
} from "../dto";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getDeviceCode(
  query: GetDeviceCodeQuery,
): Promise<DeviceCodeResponse | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/oauth/device?${queryString}`,
    { method: "POST" },
  );

  return await handleResponse(response, DeviceCodeResponseSchema);
}

export async function pollToken(
  request: PollTokenRequest,
): Promise<TokenResponse | null> {
  const response = await authPost(`${GITDOT_SERVER_URL}/oauth/token`, request);

  return await handleResponse(response, TokenResponseSchema);
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
