import { z } from "zod";
import { UploadUserImageResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UploadUserImageResponse = UploadUserImageResource;
export type UploadUserImageResponse = z.infer<typeof UploadUserImageResponse>;

export const UploadUserImage = {
  path: "/user/image",
  method: "POST",
  request: z.instanceof(Blob),
  response: UploadUserImageResponse,
} as const satisfies Endpoint;
export type UploadUserImage = typeof UploadUserImage;
