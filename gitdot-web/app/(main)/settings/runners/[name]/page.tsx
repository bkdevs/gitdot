import { notFound } from "next/navigation";

import { getCurrentUser, getRunner } from "@/dal";
import { InstallRunnerForm } from "./ui/install-runner-form";
import { RunnerDetails } from "./ui/runner-details";
import { RunnerMain } from "./ui/runner-main";

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

  return (
    <div className="flex w-full flex-col">
      <div className="flex w-full">
        <div className="flex flex-1 flex-col">
          <h1 className="flex h-9 items-center border-b border-border pl-2">
            {runner.name}
          </h1>
          <div className="flex w-full">
            <RunnerMain />
          </div>
        </div>
        <RunnerDetails runner={runner} />
      </div>
      <div className="flex border-t border-border" />
    </div>
  );
}
