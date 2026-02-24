import "server-only";

import {
  type GitHubInstallationResponse,
  GitHubInstallationResponseSchema,
} from "../dto/migration";
import { authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function createInstallation(
  installationId: number,
): Promise<GitHubInstallationResponse | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/migration/github/${installationId}`,
    {},
  );

  return await handleResponse(response, GitHubInstallationResponseSchema);
}
