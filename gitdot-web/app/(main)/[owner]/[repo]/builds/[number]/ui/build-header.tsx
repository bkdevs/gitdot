import type {
  BuildResource,
  RepositoryCommitResource,
  TaskResource,
} from "gitdot-api";
import { BuildDag } from "./build-dag";
import { BuildDetails } from "./build-details";

export function BuildHeader({
  build,
  commit,
  tasks,
}: {
  build: BuildResource;
  commit: RepositoryCommitResource | null;
  tasks: TaskResource[];
}) {
  return (
    <div className="flex h-56 border-b">
      <div className="flex w-3/4 items-center justify-center">
        <BuildDag tasks={tasks} />
      </div>
      <BuildDetails build={build} commit={commit} />
    </div>
  );
}
