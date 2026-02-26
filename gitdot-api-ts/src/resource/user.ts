import { z } from "zod";

export const UserResource = z.object({
  id: z.string().uuid(),
  name: z.string(),
  email: z.string(),
  created_at: z.string().datetime(),
});
export type UserResource = z.infer<typeof UserResource>;
