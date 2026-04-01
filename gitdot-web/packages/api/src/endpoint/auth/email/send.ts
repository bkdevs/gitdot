import { z } from "zod";
import type { Endpoint } from "../../endpoint";

export const SendAuthEmailRequest = z.object({
  email: z.string(),
});
export type SendAuthEmailRequest = z.infer<typeof SendAuthEmailRequest>;

export const SendAuthEmail = {
  path: "/auth/email/send",
  method: "POST",
  request: SendAuthEmailRequest,
  response: z.void(),
} as const satisfies Endpoint;

export type SendAuthEmail = typeof SendAuthEmail;
