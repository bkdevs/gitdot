import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const UploadUserImage = {
  path: "/user/image",
  method: "POST",
  request: z.instanceof(Blob),
  response: z.void(),
} as const satisfies Endpoint;
export type UploadUserImage = typeof UploadUserImage;
