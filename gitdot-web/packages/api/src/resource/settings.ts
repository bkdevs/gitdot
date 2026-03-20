import { z } from "zod";

export const CommitFilterResource = z.object({
  authors: z.array(z.string()).optional(),
  tags: z.array(z.string()).optional(),
  included_paths: z.array(z.string()).optional(),
  excluded_paths: z.array(z.string()).optional(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});
export type CommitFilterResource = z.infer<typeof CommitFilterResource>;
