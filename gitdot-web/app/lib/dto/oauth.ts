import { z } from "zod";

export const GetDeviceCodeQuerySchema = z.object({
  client_id: z.string(),
});

export type GetDeviceCodeQuery = z.infer<typeof GetDeviceCodeQuerySchema>;

export const DeviceCodeResponseSchema = z.object({
  device_code: z.string(),
  user_code: z.string(),
  verification_uri: z.string(),
  expires_in: z.number(),
  interval: z.number(),
});

export type DeviceCodeResponse = z.infer<typeof DeviceCodeResponseSchema>;

export const PollTokenRequestSchema = z.object({
  device_code: z.string(),
  client_id: z.string(),
});

export type PollTokenRequest = z.infer<typeof PollTokenRequestSchema>;

export const TokenResponseSchema = z.object({
  access_token: z.string(),
  user_name: z.string(),
  user_email: z.string(),
});

export type TokenResponse = z.infer<typeof TokenResponseSchema>;

export const AuthorizeDeviceRequestSchema = z.object({
  user_code: z.string(),
});

export type AuthorizeDeviceRequest = z.infer<
  typeof AuthorizeDeviceRequestSchema
>;
