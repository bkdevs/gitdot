import { notFound } from "next/navigation";

import { isRepositoryAdmin } from "@/lib/dal";

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
      <div className="flex flex-col gap-3 p-4">
        <div className="text-sm">
          <span className="text-muted-foreground">Repository:</span>{" "}
          {owner}/{repo}
        </div>
      </div>
    </div>
  );
}
