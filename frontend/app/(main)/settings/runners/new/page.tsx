import { getCurrentUser } from "@/lib/dal";
import CreateRunnerForm from "../ui/create-runner-form";
import CreateRunnerInstructions from "../ui/create-runner-instructions";

export default async function Page() {
  const user = await getCurrentUser();

  return (
    <div className="flex p-4">
      <div className="min-w-0 flex-1 max-w-3xl">
        <CreateRunnerForm user={user} />
      </div>
      <aside className="w-72 shrink-0 hidden md:block border-l border-border ml-6 pl-6">
        <CreateRunnerInstructions />
      </aside>
    </div>
  );
}
