import { CreateBuildButton } from "./create-build-button";

export function BuildsHeader({ owner, repo }: { owner: string; repo: string }) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <div className="ml-auto h-full flex flex-row">
        <CreateBuildButton owner={owner} repo={repo} />
      </div>
    </div>
  );
}
