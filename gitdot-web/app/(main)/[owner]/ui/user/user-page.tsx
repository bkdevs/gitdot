import type { UserResource } from "gitdot-api";
import {
  getCurrentUser,
  listUserCommits,
  listUserOrganizations,
  listUserRepositories,
} from "@/dal";
import { UserActions } from "./user-actions";
import { UserCommits } from "./user-commits";
import { UserLinks } from "./user-links";
import { UserOrgs } from "./user-orgs";
import { UserProfile } from "./user-profile";
import { UserReadme } from "./user-readme";
import { UserRepos } from "./user-repos";

export default async function UserPage({ user }: { user: UserResource }) {
  const [commits, repos, memberships, current] = await Promise.all([
    listUserCommits(user.name),
    listUserRepositories(user.name),
    listUserOrganizations(user.name),
    getCurrentUser(false),
  ]);

  const isOwner = current?.user.name === user.name;

  return (
    <div className="grid grid-cols-[8rem_minmax(0,3fr)_minmax(0,2fr)] h-full">
      <div className="overflow-y-auto scrollbar-none">
        <div className="flex flex-col items-start pl-4 pr-2 my-2.5 pt-0.5 gap-6">
          <UserProfile user={user} />
          <UserLinks user={user} />
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
