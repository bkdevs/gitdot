import Link from "@/ui/link";

const navItems = [
  { path: "", label: "/home" },
  { path: "files", label: "/files" },
  { path: "commits", label: "/commits" },
  { path: "questions", label: "/questions" },
  { path: "builds", label: "/builds" },
];

export function RepoSidebarNav({
  owner,
  repo,
  currentPath,
  showSettings,
}: {
  owner: string;
  repo: string;
  currentPath: string;
  showSettings?: boolean;
}) {
  const items = showSettings
    ? [...navItems, { path: "settings", label: "/settings" }]
    : navItems;
  const isActive = (itemPath: string) => {
    if (itemPath === "") {
      return currentPath === "/" || currentPath === "";
    }
    return (
      currentPath === `/${itemPath}` || currentPath.startsWith(`/${itemPath}/`)
    );
  };

  return (
    <div className="flex flex-col w-full">
      {items.map((item) => {
        const active = isActive(item.path);
        return (
          <Link
            key={item.label}
            href={
              item.path ? `/${owner}/${repo}/${item.path}` : `/${owner}/${repo}`
            }
            className={`flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm hover:bg-accent/50 font-mono ${
              active ? "bg-sidebar" : ""
            }`}
            prefetch={true}
          >
            <span className="ml-2">{item.label}</span>
          </Link>
        );
      })}
    </div>
  );
}
