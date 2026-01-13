import { CommitsCalendar } from "./ui/commits-calendar";
import type { RepositoryCommit } from "@/app/lib/dto";

// Generate mock commits for the past year
function generateMockYearCommits(): RepositoryCommit[] {
  const commits: RepositoryCommit[] = [];
  const endDate = new Date();
  const authors = [
    "Alice Johnson",
    "Bob Smith",
    "Charlie Brown",
    "Diana Prince",
  ];
  const messageTemplates = [
    "Fix authentication bug in login flow",
    "Add new feature for user profiles",
    "Update dependencies and fix vulnerabilities",
    "Refactor code for better performance",
    "Add tests for API endpoints",
    "Update documentation",
    "Fix styling issues on mobile",
    "Implement dark mode support",
    "Optimize database queries",
    "Add error handling for edge cases",
    "Update UI components",
    "Fix memory leak in component",
    "Add accessibility improvements",
    "Update build configuration",
    "Fix race condition in async code",
  ];

  // Generate commits for the past 365 days
  for (let i = 0; i < 365; i++) {
    const date = new Date(endDate);
    date.setDate(date.getDate() - i);

    // Vary commit frequency: more commits on weekdays, fewer on weekends
    const dayOfWeek = date.getDay();
    const isWeekend = dayOfWeek === 0 || dayOfWeek === 6;

    // Random 0-8 commits per weekday, 0-3 on weekends
    const maxCommits = isWeekend ? 3 : 8;
    const commitCount = Math.floor(Math.random() * (maxCommits + 1));

    for (let j = 0; j < commitCount; j++) {
      // Random time during the day
      const hour = 9 + Math.floor(Math.random() * 9); // 9 AM - 5 PM
      const minute = Math.floor(Math.random() * 60);
      date.setHours(hour, minute, 0, 0);

      commits.push({
        sha: `${i.toString(16).padStart(8, "0")}${j.toString(16)}`,
        message:
          messageTemplates[
            Math.floor(Math.random() * messageTemplates.length)
          ],
        author: authors[Math.floor(Math.random() * authors.length)],
        date: date.toISOString(),
      });
    }
  }

  return commits;
}

export default function Page() {
  const mockCommits = generateMockYearCommits();

  return <CommitsCalendar commits={mockCommits} />;
}
