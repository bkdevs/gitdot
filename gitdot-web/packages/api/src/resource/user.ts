import { z } from "zod";
import { CommitFilterResource } from "./settings";

export const UserResource = z.object({
  id: z.uuid(),
  name: z.string(),
  email: z.string(),
  created_at: z.iso.datetime(),
  location: z.string().optional(),
  readme: z.string().nullable().optional(),
  website: z.string().nullable().optional(),
});
export type UserResource = z.infer<typeof UserResource>;

export const UserRepoSettingsResource = z.object({
  commit_filters: z.array(CommitFilterResource).optional(),
});
export type UserRepoSettingsResource = z.infer<typeof UserRepoSettingsResource>;

export const UserSettingsResource = z.object({
  repos: z.record(z.string(), UserRepoSettingsResource),
});
export type UserSettingsResource = z.infer<typeof UserSettingsResource>;
