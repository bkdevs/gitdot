import { CreateTaskButton } from "./create-task-button";

export function TasksHeader({ owner, repo }: { owner: string; repo: string }) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <div className="ml-auto h-full flex flex-row">
        <CreateTaskButton owner={owner} repo={repo} />
      </div>
    </div>
  );
}
