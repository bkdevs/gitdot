import { z } from "zod";
import { DeviceCodeResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const CreateDeviceCodeRequest = z.object({
  client_id: z.string(),
});
export type CreateDeviceCodeRequest = z.infer<typeof CreateDeviceCodeRequest>;

export const CreateDeviceCodeResponse = DeviceCodeResource;
export type CreateDeviceCodeResponse = z.infer<typeof CreateDeviceCodeResponse>;

export const CreateDeviceCode = {
  path: "/auth/device/code",
  method: "POST",
  request: CreateDeviceCodeRequest,
  response: CreateDeviceCodeResponse,
} as const satisfies Endpoint;

export type CreateDeviceCode = typeof CreateDeviceCode;
