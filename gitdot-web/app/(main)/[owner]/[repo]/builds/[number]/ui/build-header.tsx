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
  configHtml,
}: {
  build: BuildResource;
  commit: RepositoryCommitResource | null;
  tasks: TaskResource[];
  configHtml: string | null;
}) {
  return (
    <div className="flex h-56 border-b">
      <div className="flex w-3/4 items-center justify-center">
        <BuildDag tasks={tasks} />
      </div>
      <BuildDetails build={build} commit={commit} configHtml={configHtml} />
    </div>
  );
}
