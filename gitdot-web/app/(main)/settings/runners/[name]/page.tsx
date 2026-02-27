import { notFound } from "next/navigation";

import { getCurrentUser, getRunner } from "@/dal";
import { formatDateTime, timeAgoFull } from "@/util";
import { InstallRunnerForm } from "./ui/install-runner-form";
import { RunnerStatus } from "./ui/runner-status";

export default async function Page({
  params,
}: {
  params: Promise<{ name: string }>;
}) {
  const user = await getCurrentUser();
  if (!user) notFound();

  const { name } = await params;
  const runner = await getRunner(user.name, name);
  if (!runner) notFound();

  if (!runner.last_active) {
    return <InstallRunnerForm runner={runner} ownerName={user.name} />;
  }

  const createdAt = new Date(runner.created_at);
  const lastActive = new Date(runner.last_active);
  const isActive = Date.now() - lastActive.getTime() <= 90 * 1000;

  return (
    <div>
      <h1 className="flex items-center border-b border-border pl-2 h-9">
        {runner.name}
      </h1>
      <div className="p-2">
        <ul className="text-sm text-muted-foreground space-y-1">
          <li>
            <RunnerStatus runner={runner} />
          </li>
          <li>
            <span>Owner:</span> {runner.owner_name}</li>
          <li>
            <span>Created:</span>{" "}
            {formatDateTime(createdAt)}
          </li>
        </ul>
      </div>
    </div>
  );
}
