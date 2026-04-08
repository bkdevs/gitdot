import { UserCommits } from "./ui/user-commits";
import { UserProfile } from "./ui/user-profile";
import { UserReadme } from "./ui/user-readme";
import { UserRepos } from "./ui/user-repos";
import { UserStatistics } from "./ui/user-statistics";

const FAKE_REPOS = [
  { name: "gitdot", stars: 847 },
  { name: "s2-sdk-rs", stars: 312 },
  { name: "axum-auth", stars: 204 },
  { name: "pg-migrate", stars: 178 },
  { name: "dotfiles", stars: 94 },
  { name: "advent-of-code", stars: 41 },
  { name: "til", stars: 29 },
  { name: "scratchpad", stars: 4 },
];

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;

  return (
    <div className="grid grid-cols-[1fr_600px_1fr] items-start py-10">
      <div className="border-r flex flex-col items-end px-4 gap-6">
        <UserProfile owner={owner}/>
        <UserRepos owner={owner}/>
      </div>

      <div className="px-2 flex flex-col gap-8">
        <UserReadme />
        <UserStatistics />
        <UserCommits />
      </div>

      <div />
    </div>
  );
}
