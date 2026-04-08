import { z } from "zod";
import { CommitFilterResource } from "./settings";

export const UserResource = z.object({
  id: z.uuid(),
  name: z.string(),
  email: z.string(),
  created_at: z.iso.datetime(),
  location: z.string().nullable().optional(),
  readme: z.string().nullable().optional(),
  links: z.array(z.string()).default([]),
  company: z.string().nullable().optional(),
  image: z.string().nullable().optional(),
});
export type UserResource = z.infer<typeof UserResource>;

export const UploadUserImageResource = z.object({
  bytes: z.string(),
});
export type UploadUserImageResource = z.infer<typeof UploadUserImageResource>;

export const UserRepoSettingsResource = z.object({
  commit_filters: z.array(CommitFilterResource).optional(),
});
export type UserRepoSettingsResource = z.infer<typeof UserRepoSettingsResource>;

export const UserSettingsResource = z.object({
  repos: z.record(z.string(), UserRepoSettingsResource),
});
export type UserSettingsResource = z.infer<typeof UserSettingsResource>;
