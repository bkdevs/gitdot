import { getUser, listUserCommits } from "@/dal";
import { getUserMetadata } from "@/lib/auth";
import Link from "@/ui/link";
import { UserCommits } from "./ui/user-commits";
import { UserLinks } from "./ui/user-links";
import { UserProfile } from "./ui/user-profile";
import { UserReadme } from "./ui/user-readme";
import { UserRepos } from "./ui/user-repos";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  const [user, commits, metadata] = await Promise.all([
    getUser(owner),
    listUserCommits(owner),
    getUserMetadata(),
  ]);
  const isCurrentUser = metadata.username === owner;

  if (!user) {
    return <div className="p-2 text-sm">{owner} not found</div>;
  }

  return (
    <div className="grid grid-cols-[1fr_600px_1fr] items-start py-10">
      <div className="border-r flex flex-col items-end px-4 gap-6">
        <UserProfile user={user} />
        <UserLinks user={user} />
        <UserRepos owner={owner} />
        {isCurrentUser && (
          <div className="flex flex-col items-end">
            <p className="font-semibold text-sm mb-0.5">settings</p>
            <Link
              href="/settings"
              className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200"
            >
              profile
            </Link>
          </div>
        )}
      </div>

      <div className="px-2 flex flex-col gap-8">
        <UserReadme readme={user.readme} />
        <UserCommits commits={commits ?? []} />
      </div>

      <div />
    </div>
  );
}
