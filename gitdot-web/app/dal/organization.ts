import "server-only";

import {
  OrganizationMemberResource,
  OrganizationResource,
  RepositoryResource,
} from "gitdot-api";
import { z } from "zod";
import {
  authFetch,
  authPatch,
  authPost,
  GITDOT_SERVER_URL,
  handleResponse,
} from "./util";

export async function getOrganization(
  name: string,
): Promise<OrganizationResource | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/organization/${name}`);
  return await handleResponse(response, OrganizationResource);
}

export async function listOrganizationMembers(
  name: string,
): Promise<OrganizationMemberResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/organization/${name}/members`,
  );
  return await handleResponse(response, z.array(OrganizationMemberResource));
}

export async function listOrganizationRepositories(
  name: string,
): Promise<RepositoryResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/organization/${name}/repositories`,
  );
  return await handleResponse(response, z.array(RepositoryResource));
}

export async function createOrganization(
  name: string,
  readme?: string,
): Promise<OrganizationResource | null> {
  const response = await authPost(`${GITDOT_SERVER_URL}/organization/${name}`, {
    readme,
  });
  return await handleResponse(response, OrganizationResource);
}

export async function updateOrganization(
  name: string,
  request: {
    location?: string | null;
    readme?: string | null;
    links?: string[];
  },
): Promise<OrganizationResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/organization/${name}`,
    request,
  );
  return await handleResponse(response, OrganizationResource);
}

export async function uploadOrganizationImage(
  orgName: string,
  file: File,
): Promise<boolean> {
  const bytes = await file.arrayBuffer();
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/organization/${orgName}/image`,
    {
      method: "POST",
      headers: { "Content-Type": file.type },
      body: bytes,
    },
  );
  return response.ok;
}
