import type { UserResource } from "gitdot-api";
import {
  listUserCommits,
  listUserOrganizations,
  listUserRepositories,
} from "@/dal";
import { getUserMetadata } from "@/lib/auth";
import { UserActions } from "./user-actions";
import { UserCommits } from "./user-commits";
import { UserLinks } from "./user-links";
import { UserOrgs } from "./user-orgs";
import { UserProfile } from "./user-profile";
import { UserReadme } from "./user-readme";
import { UserRepos } from "./user-repos";

export default async function UserPage({ user }: { user: UserResource }) {
  const [commits, repos, orgs, metadata] = await Promise.all([
    listUserCommits(user.name),
    listUserRepositories(user.name),
    listUserOrganizations(user.name),
    getUserMetadata(),
  ]);

  const isOwner = metadata.username === user.name;

  return (
    <div className="grid grid-cols-[8rem_minmax(0,3fr)_minmax(0,2fr)] h-full">
      <div className="overflow-y-auto scrollbar-none">
        <div className="flex flex-col items-end px-4 my-2.5 pt-0.5 gap-6 border-r">
          <UserProfile user={user} />
          <UserLinks user={user} />
          {isOwner && <UserActions />}
        </div>
      </div>

      <div className="pl-4 pr-6 py-2 flex flex-col gap-8 overflow-y-auto scrollbar-none">
        <UserReadme readme={user.readme} />
        <UserOrgs orgs={orgs} />
        <UserRepos repos={repos} />
      </div>

      <div className="pt-2 border-l flex flex-col min-h-0">
        <UserCommits commits={commits ?? []} />
      </div>
    </div>
  );
}
