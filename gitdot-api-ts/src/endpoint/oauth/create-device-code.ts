import { z } from "zod";

import { DeviceCodeResource } from "../../resource";

export const CreateDeviceCodeRequest = z.object({
  client_id: z.string(),
});
export type CreateDeviceCodeRequest = z.infer<typeof CreateDeviceCodeRequest>;

export const CreateDeviceCode = {
  path: "/oauth/device",
  method: "POST",
  request: CreateDeviceCodeRequest,
  response: DeviceCodeResource,
} as const;
export type CreateDeviceCode = typeof CreateDeviceCode;
