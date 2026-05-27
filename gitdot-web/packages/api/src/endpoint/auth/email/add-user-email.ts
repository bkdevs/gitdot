import { z } from "zod";
import { UserEmailResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const AddUserEmailRequest = z.object({
  email: z.string(),
});
export type AddUserEmailRequest = z.infer<typeof AddUserEmailRequest>;

export const AddUserEmailResponse = UserEmailResource;
export type AddUserEmailResponse = z.infer<typeof AddUserEmailResponse>;

export const AddUserEmail = {
  path: "/auth/email/add-email",
  method: "POST",
  request: AddUserEmailRequest,
  response: AddUserEmailResponse,
} as const satisfies Endpoint;
export type AddUserEmail = typeof AddUserEmail;
