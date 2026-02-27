import { notFound } from "next/navigation";

import { isRepositoryAdmin } from "@/dal";
import { DeleteRepositoryButton } from "./ui/delete-repository-button";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;

  const isAdmin = await isRepositoryAdmin(owner, repo);
  if (!isAdmin) notFound();

  return (
    <div className="flex flex-col w-full">
      <div className="flex items-center border-b pl-2 h-9">
        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          General
        </h3>
      </div>
      <div className="flex flex-col gap-4 p-4">
        <div className="flex flex-col gap-2">
          <h4 className="text-sm font-medium text-destructive">Danger Zone</h4>
          <p className="text-xs text-muted-foreground">
            Once you delete a repository, there is no going back.
          </p>
          <div>
            <DeleteRepositoryButton owner={owner} repo={repo} />
          </div>
        </div>
      </div>
    </div>
  );
}
