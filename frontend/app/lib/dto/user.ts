import { z } from "zod";

export const UpdateCurrentUserRequestSchema = z.object({
  name: z.string(),
});

export type UpdateCurrentUserRequest = z.infer<typeof UpdateCurrentUserRequestSchema>;

export const UserResponseSchema = z.object({
  id: z.uuid(),
  name: z.string(),
  email: z.string(),
  created_at: z.iso.datetime(),
});

export type UserResponse = z.infer<typeof UserResponseSchema>;

export const UserRepositoryResponseSchema = z.object({
  id: z.uuid(),
  name: z.string(),
  owner: z.string(),
  visibility: z.string(),
  created_at: z.iso.datetime(),
});

export type UserRepositoryResponse = z.infer<
  typeof UserRepositoryResponseSchema
>;

export const UserRepositoriesResponseSchema = z.array(
  UserRepositoryResponseSchema,
);

export type UserRepositoriesResponse = z.infer<
  typeof UserRepositoriesResponseSchema
>;
