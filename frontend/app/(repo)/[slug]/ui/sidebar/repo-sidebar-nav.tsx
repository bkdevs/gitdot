import { Code2, GitCommit, Home, MessageCircleQuestion } from "lucide-react";
import Link from "next/link";

const navItems = [
  { path: "", label: "Home", icon: Home },
  { path: "files", label: "Files", icon: Code2 },
  { path: "commits", label: "Commits", icon: GitCommit },
  { path: "questions", label: "Questions", icon: MessageCircleQuestion },
];

export function RepoSidebarNav({
  repo,
  currentPath,
}: {
  repo: string;
  currentPath: string;
}) {
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
      {navItems.map((item) => {
        const active = isActive(item.path);
        return (
          <Link
            key={item.label}
            href={item.path ? `/${repo}/${item.path}` : `/${repo}`}
            className={`flex flex-row w-full px-2 h-9 items-center border-b select-none cursor-default text-sm hover:bg-accent/50 ${
              active ? "bg-sidebar" : ""
            }`}
            prefetch={true}
          >
            <item.icon className="size-4" />
            <span className="ml-2">{item.label}</span>
          </Link>
        );
      })}
    </div>
  );
}
