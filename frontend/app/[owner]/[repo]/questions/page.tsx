import Link from "next/link";

const EXAMPLE_QUESTIONS = [
  { id: 1, title: "Fix authentication bug in login flow" },
  { id: 2, title: "Add dark mode support" },
  { id: 3, title: "Improve performance of data grid" },
  { id: 4, title: "Update dependencies to latest versions" },
  { id: 5, title: "Add unit tests for API endpoints" },
  { id: 6, title: "Refactor component structure" },
  { id: 7, title: "Fix mobile responsiveness issues" },
  { id: 8, title: "Add TypeScript strict mode" },
  { id: 9, title: "Implement user profile page" },
  { id: 10, title: "Fix memory leak in dashboard" },
  { id: 11, title: "Add documentation for API" },
  { id: 12, title: "Improve error handling" },
];

export default async function Page({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug: repo } = await params;

  return (
    <div className="flex flex-col">
      {EXAMPLE_QUESTIONS.map((question) => (
        <Link
          key={question.id}
          href={`/${repo}/questions/${question.id}`}
          className="flex w-full border-b hover:bg-accent/50 select-none cursor-default py-2 px-2"
          prefetch={true}
        >
          <div className="flex flex-col w-full justify-start items-start min-w-0">
            <div className="text-sm truncate w-full">
              #{question.id}: {question.title}
            </div>
          </div>
        </Link>
      ))}
    </div>
  );
}
