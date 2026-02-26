import { z } from "zod";

import { VoteResource } from "../../resource";

export const VoteCommentRequest = z.object({
  value: z.number().int(),
});
export type VoteCommentRequest = z.infer<typeof VoteCommentRequest>;

export const VoteComment = {
  path: "/repository/{owner}/{repo}/question/{number}/comment/{comment_id}/vote",
  method: "POST",
  request: VoteCommentRequest,
  response: VoteResource,
} as const;
export type VoteComment = typeof VoteComment;
