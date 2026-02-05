import { z } from "zod";

export const CreateUserRequestSchema = z.object({
  name: z.string(),
  email: z.string(),
  password: z.string(),
});

export type CreateUserRequest = z.infer<typeof CreateUserRequestSchema>;

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
