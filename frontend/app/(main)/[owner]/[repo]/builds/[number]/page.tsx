import { getBuildByNumber } from "@/lib/dal";
import { TaskRow } from "./ui/task-row";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
}) {
  const { owner, repo, number: numberStr } = await params;
  const number = Number(numberStr);
  if (Number.isNaN(number)) return null;

  const data = await getBuildByNumber(owner, repo, number);
  if (!data) return null;

  const { build, tasks } = data;

  return (
    <div className="flex flex-col">
      <div className="border-b py-2 px-3 text-sm text-muted-foreground">
        <span className="font-mono">{build.commit_sha.slice(0, 7)}</span>
        <span className="ml-2">{build.trigger}</span>
      </div>
      {tasks.map((task) => (
        <TaskRow key={task.id} task={task} />
      ))}
    </div>
  );
}
