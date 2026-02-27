import { z } from "zod";

export const UserResource = z.object({
  id: z.uuid(),
  name: z.string(),
  email: z.string(),
  created_at: z.iso.datetime(),
});
export type UserResource = z.infer<typeof UserResource>;
