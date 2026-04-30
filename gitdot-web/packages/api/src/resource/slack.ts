import { z } from "zod";

export const SlackAccountResource = z.object({
  id: z.uuid(),
  gitdot_user_id: z.uuid(),
  slack_user_id: z.string(),
  slack_team_id: z.string(),
  created_at: z.iso.datetime(),
});
export type SlackAccountResource = z.infer<typeof SlackAccountResource>;
