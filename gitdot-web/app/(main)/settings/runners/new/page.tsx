import { notFound } from "next/navigation";
import { getCurrentUser } from "@/dal";
import { CreateRunnerForm } from "../ui/create-runner-form";
import { CreateRunnerInstructions } from "../ui/create-runner-instructions";

export default async function Page() {
  const user = await getCurrentUser();
  if (!user) notFound();

  return (
    <div className="flex p-4">
      <div className="min-w-0 flex-1 max-w-3xl">
        <CreateRunnerForm ownerName={user.name} ownerType="user" />
      </div>
      <aside className="w-72 shrink-0 hidden md:block border-l border-border ml-6 pl-6">
        <CreateRunnerInstructions />
      </aside>
    </div>
  );
}
