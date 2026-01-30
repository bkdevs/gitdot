import "server-only";

import { z } from "zod";
import { API_BASE_URL, authPost, handleResponse } from "./util";

const DeviceAuthorizationSchema = z.object({
  user_code: z.string(),
  status: z.string(),
});

export type DeviceAuthorization = z.infer<typeof DeviceAuthorizationSchema>;

export async function authorizeDevice(userCode: string): Promise<boolean> {
  const response = await authPost(`${API_BASE_URL}/oauth/authorize`, {
    user_code: userCode,
  });

  return response.ok;
}
