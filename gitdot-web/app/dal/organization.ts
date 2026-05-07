import "server-only";

import { OrganizationResource } from "gitdot-api";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getOrganization(
  name: string,
): Promise<OrganizationResource | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/organization/${name}`);
  return await handleResponse(response, OrganizationResource);
}

export async function createOrganization(
  name: string,
): Promise<OrganizationResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/organization/${name}`,
    {},
  );
  return await handleResponse(response, OrganizationResource);
}
