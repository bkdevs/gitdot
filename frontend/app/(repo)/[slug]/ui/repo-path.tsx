import Link from "next/link";

const MAX_CHARACTERS = 32;

export function RepoPath({
  repo,
  currentPath,
}: {
  repo: string;
  currentPath: string;
}) {
  const pathSegments = currentPath.split("/").slice(0, -1);
  const pathLinks: React.ReactNode[] = [
    <Link className="hover:underline" href={`/${repo}`} key="repo-root">
      {repo}
    </Link>,
  ];

  let remainingCharacters = MAX_CHARACTERS - repo.length;
  let totalChars = 0;
  for (const segment of pathSegments) {
    totalChars += segment.length + 1;
  }
  if (totalChars > remainingCharacters) {
    // truncating, reserve 3 chars for "/.."
    remainingCharacters -= 3;
  }

  let charsNeeded = 0;
  let segmentsFit = 0;

  for (let i = pathSegments.length - 1; i >= 0; i--) {
    const segment = pathSegments[i];
    const segmentChars = segment.length + 1;
    if (charsNeeded + segmentChars > remainingCharacters) {
      break;
    }
    charsNeeded += segmentChars;
    segmentsFit++;
  }

  const startIndex = pathSegments.length - segmentsFit;
  let truncatedPath = "";
  for (let i = 0; i < startIndex; i++) {
    truncatedPath += `/${pathSegments[i]}`;
  }

  if (startIndex > 0) {
    pathLinks.push(<span key="separator-truncate">/</span>);
    pathLinks.push(
      <Link
        className="hover:underline"
        href={`/${repo}${truncatedPath}`}
        key="truncate"
      >
        ..
      </Link>,
    );
  }

  let path = truncatedPath;
  for (let i = startIndex; i < pathSegments.length; i++) {
    const segment = pathSegments[i];
    path += `/${segment}`;
    pathLinks.push(<span key={`${segment}-separator-${i}`}>/</span>);
    pathLinks.push(
      <Link
        className="hover:underline"
        href={`/${repo}${path}`}
        key={`${segment}-${i}`}
      >
        {segment}
      </Link>,
    );
  }

  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <div className="flex-1 ml-2 text-sm text-nowrap">{pathLinks}</div>
    </div>
  );
}
