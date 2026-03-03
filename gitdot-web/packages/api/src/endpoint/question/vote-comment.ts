import { z } from "zod";
import { VoteResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const VoteCommentRequest = z.object({
  value: z.number().int(),
});
export type VoteCommentRequest = z.infer<typeof VoteCommentRequest>;

export const VoteCommentResponse = VoteResource;
export type VoteCommentResponse = z.infer<typeof VoteCommentResponse>;

export const VoteComment = {
  path: "/repository/{owner}/{repo}/question/{number}/comment/{comment_id}/vote",
  method: "POST",
  request: VoteCommentRequest,
  response: VoteCommentResponse,
} as const satisfies Endpoint;
export type VoteComment = typeof VoteComment;
