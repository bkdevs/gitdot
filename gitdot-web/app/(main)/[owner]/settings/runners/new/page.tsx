import { notFound } from "next/navigation";
import { CreateRunnerForm } from "@/(main)/settings/runners/ui/create-runner-form";
import { CreateRunnerInstructions } from "@/(main)/settings/runners/ui/create-runner-instructions";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  if (!owner) notFound();

  return (
    <div className="flex p-4">
      <div className="min-w-0 flex-1 max-w-3xl">
        <CreateRunnerForm ownerName={owner} ownerType="organization" />
      </div>
      <aside className="w-72 shrink-0 hidden md:block border-l border-border ml-6 pl-6">
        <CreateRunnerInstructions />
      </aside>
    </div>
  );
}
