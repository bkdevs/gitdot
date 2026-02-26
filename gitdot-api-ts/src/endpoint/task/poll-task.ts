import { z } from "zod";
import { PollTaskResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const PollTaskRequest = z.object({});
export type PollTaskRequest = z.infer<typeof PollTaskRequest>;

export const PollTaskResponse = PollTaskResource.nullable();
export type PollTaskResponse = z.infer<typeof PollTaskResponse>;

export const PollTask = {
  path: "/ci/task/poll",
  method: "GET",
  request: PollTaskRequest,
  response: PollTaskResponse,
} satisfies Endpoint;
export type PollTask = typeof PollTask;
