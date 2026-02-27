import { notFound } from "next/navigation";
import { InstallRunnerForm } from "@/(main)/settings/runners/[name]/ui/install-runner-form";
import { RunnerDetails } from "@/(main)/settings/runners/[name]/ui/runner-details";
import { RunnerMain } from "@/(main)/settings/runners/[name]/ui/runner-main";
import { getRunner } from "@/dal";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; name: string }>;
}) {
  const { owner, name } = await params;
  const runner = await getRunner(owner, name);
  if (!runner) notFound();

  if (!runner.last_active) {
    return <InstallRunnerForm runner={runner} ownerName={owner} />;
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
