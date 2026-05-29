import type { UserResource } from "gitdot-api";
import {
  getCurrentUser,
  listUserCommits,
  listUserOrganizations,
  listUserRepositories,
  listUserStarredRepositories,
} from "gitdot-client";
import { UserActions } from "./user-actions";
import { UserCommits } from "./user-commits";
import { UserLinks } from "./user-links";
import { UserOrgs } from "./user-orgs";
import { UserProfile } from "./user-profile";
import { UserReadme } from "./user-readme";
import { UserRepos } from "./user-repos";
import { UserStars } from "./user-stars";

export default async function UserPage({ user }: { user: UserResource }) {
  const [
    commitsResponse,
    reposResponse,
    membershipsResponse,
    starsResponse,
    current,
  ] = await Promise.all([
    listUserCommits(user.name),
    listUserRepositories(user.name),
    listUserOrganizations(user.name),
    listUserStarredRepositories(user.name),
    getCurrentUser(false),
  ]);
  const commits = commitsResponse?.data ?? null;
  const repos = reposResponse?.data ?? null;
  const memberships = membershipsResponse?.data ?? null;
  const stars = starsResponse?.data ?? null;
  const isOwner = current?.name === user.name;

  return (
    <div className="grid grid-cols-[15rem_minmax(0,3fr)_minmax(0,2fr)] h-full">
      <div className="overflow-y-auto scrollbar-none">
        <div className="flex flex-col items-end pl-2 pr-4 my-2.5 pt-0.5 gap-6">
          <UserProfile user={user} />
          <UserLinks user={user} />
          <UserStars stars={stars ?? []} />
          {isOwner && <UserActions />}
        </div>
      </div>

      <div className="border-l px-3 py-2 flex flex-col gap-8 overflow-y-auto scrollbar-none">
        <UserReadme readme={user.readme} />
        <UserOrgs memberships={memberships} />
        <UserRepos repos={repos} commits={commits ?? []} isOwner={isOwner} />
      </div>

      <div className="pt-2 border-l flex flex-col min-h-0">
        <UserCommits commits={commits ?? []} />
      </div>
    </div>
  );
}
