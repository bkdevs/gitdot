import { z } from "zod";

export const CommitFilterResource = z.object({
  name: z.string(),
  authors: z.array(z.string()).nullable().optional(),
  tags: z.array(z.string()).nullable().optional(),
  included_paths: z.array(z.string()).nullable().optional(),
  excluded_paths: z.array(z.string()).nullable().optional(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});
export type CommitFilterResource = z.infer<typeof CommitFilterResource>;
