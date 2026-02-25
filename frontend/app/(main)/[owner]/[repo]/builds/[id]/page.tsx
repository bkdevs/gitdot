import { getBuildTasks } from "@/lib/dal";
import { TaskRow } from "./ui/task-row";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; id: string }>;
}) {
  const { id } = await params;
  const tasks = await getBuildTasks(id);
  if (!tasks) return null;

  return (
    <div className="flex flex-col">
      {tasks.map((task) => (
        <TaskRow key={task.id} task={task} />
      ))}
    </div>
  );
}
