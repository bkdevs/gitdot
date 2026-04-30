import { z } from "zod";
import { SlackAccountResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const LinkSlackAccountRequest = z.object({
  state: z.string(),
});
export type LinkSlackAccountRequest = z.infer<typeof LinkSlackAccountRequest>;

export const LinkSlackAccountResponse = SlackAccountResource;
export type LinkSlackAccountResponse = z.infer<typeof LinkSlackAccountResponse>;

export const LinkSlackAccount = {
  path: "/auth/slack/link",
  method: "POST",
  request: LinkSlackAccountRequest,
  response: LinkSlackAccountResponse,
} as const satisfies Endpoint;

export type LinkSlackAccount = typeof LinkSlackAccount;
