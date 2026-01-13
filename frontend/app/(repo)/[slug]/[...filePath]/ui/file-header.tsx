import Link from "next/link";

export function FileHeader({
  repo,
  filePath,
}: {
  repo: string;
  filePath: string;
}) {
  var path = "";
  const pathSegments = filePath.split("/");
  const pathLinks: React.ReactNode[] = [
    <Link
      className="hover:underline"
      href={`/${repo}`}
      key="repo-root"
      prefetch={true}
    >
      {repo}
    </Link>,
    <span key="repo-separator">/</span>,
  ];

  pathSegments.forEach((segment, index) => {
    path += `/${segment}`;
    pathLinks.push(
      <Link
        className="hover:underline"
        href={`/${repo}${path}`}
        key={segment}
        prefetch={true}
      >
        {segment}
      </Link>,
    );
    if (index !== pathSegments.length - 1) {
      pathLinks.push(<span key={`${segment}-separator`}>/</span>);
    }
  });

  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <div className="flex-1 ml-2 text-sm font-mono">{pathLinks}</div>
      <div className="border-l w-64 flex flex-row items-center h-full pl-2 text-sm">
        Commits
      </div>
    </div>
  );
}
