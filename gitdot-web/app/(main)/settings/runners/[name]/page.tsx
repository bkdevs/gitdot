import { notFound } from "next/navigation";

import { getCurrentUser, getRunner } from "@/dal";
import { VerifyRunnerForm } from "./ui/verify-runner-form";
import { VerifyRunnerInstructions } from "./ui/verify-runner-instructions";

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

  if (runner.last_verified) {
    return (
      <div className="p-4">
        <h1 className="text-lg font-medium border-b border-border pb-2 mb-4">
          {runner.name}
        </h1>
        <p className="text-sm text-muted-foreground">
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
          eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad
          minim veniam, quis nostrud exercitation ullamco laboris nisi ut
          aliquip ex ea commodo consequat.
        </p>
      </div>
    );
  }

  return (
    <div className="flex p-4">
      <div className="min-w-0 flex-1 max-w-3xl">
        <VerifyRunnerForm runner={runner} ownerName={user.name} />
      </div>
      <aside className="w-72 shrink-0 hidden md:block border-l border-border ml-6 pl-6">
        <VerifyRunnerInstructions />
      </aside>
    </div>
  );
}
