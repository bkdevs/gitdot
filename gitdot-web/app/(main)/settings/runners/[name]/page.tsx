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
    <div className="flex flex-col w-full">
      <h1 className="flex items-center border-b border-border pl-2 h-9">
        {runner.name}
      </h1>
      <div className="flex w-full">
        <RunnerMain />
        <RunnerDetails runner={runner} />
      </div>
      <div className="flex border-t border-border" />

    </div>
  );
}
