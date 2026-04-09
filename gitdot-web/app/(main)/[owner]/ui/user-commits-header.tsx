"use client";

export function UserCommitsHeader({
  startDate,
  endDate,
  selectedMonth,
}: {
  startDate: string;
  endDate: string;
  selectedMonth: string | null;
}) {
  return (
    <div className="flex items-baseline mb-2 justify-between">
      <span className="text-xs text-muted-foreground font-mono">
        <span className="text-foreground/40 select-none"># </span>Activity
      </span>
      <span className="text-xs text-muted-foreground/60 font-mono">
        {selectedMonth
          ? formatYearMonth(selectedMonth)
          : `${formatYearMonth(startDate.slice(0, 7))} - ${formatYearMonth(endDate.slice(0, 7))}`}
      </span>
    </div>
  );
}

function formatYearMonth(yearMonth: string): string {
  const [year, month] = yearMonth.split("-").map(Number);
  return new Date(year, month - 1).toLocaleString("en-US", {
    month: "short",
    year: "numeric",
  });
}
