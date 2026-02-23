import { getTasks } from "@/lib/dal";
import { TasksClient } from "./ui/tasks-client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const tasks = await getTasks(owner, repo);
  if (!tasks) return null;

  return <TasksClient owner={owner} repo={repo} tasks={tasks} />;
}
