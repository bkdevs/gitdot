import Link from "next/link";

export function RepoPath({
  repo,
  currentPath,
}: {
  repo: string;
  currentPath: string;
}) {
  var path = "";
  const pathSegments = currentPath.split("/").slice(0, -1);
  const pathLinks: React.ReactNode[] = [];

  if (pathSegments.length === 0) {
    pathLinks.push(
      <Link className="hover:underline" href={`/${repo}`} key="repo-root">
        {repo}
      </Link>,
    );
  } else {
    pathLinks.push(
      <Link className="hover-underline" href={`/${repo}`} key="home">
        ~
      </Link>,
    );
  }

  pathSegments.forEach((segment) => {
    path += `/${segment}`;
    pathLinks.push(<span key={`${segment}-separator`}>/</span>);
    pathLinks.push(
      <Link className="hover:underline" href={`/${repo}${path}`} key={segment}>
        {segment}
      </Link>,
    );
  });

  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <div className="flex-1 ml-2 text-sm font-mono">{pathLinks}</div>
    </div>
  );
}
