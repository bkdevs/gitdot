import { getUser, listUserCommits } from "@/dal";
import { UserCommits } from "./ui/user-commits";
import { UserProfile } from "./ui/user-profile";
import { UserReadme } from "./ui/user-readme";
import { UserRepos } from "./ui/user-repos";
import { UserStatistics } from "./ui/user-statistics";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  const [user, commits] = await Promise.all([
    getUser(owner),
    listUserCommits(owner),
  ]);

  if (!user) {
    return <div className="p-2 text-sm">{owner} not found</div>;
  }

  return (
    <div className="grid grid-cols-[1fr_600px_1fr] items-start py-10">
      <div className="border-r flex flex-col items-end px-4 gap-6">
        <UserProfile user={user}/>
        <UserRepos owner={owner}/>
      </div>

      <div className="px-2 flex flex-col gap-8">
        <UserReadme />
        <UserStatistics />
        <UserCommits commits={commits ?? []} />
      </div>

      <div />
    </div>
  );
}
