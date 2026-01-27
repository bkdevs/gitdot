import {
  ChevronDown,
  Plus,
  Square,
  SquareDashed,
  SquareDot,
} from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { cn } from "@/util";
import type { QuestionsFilter, QuestionsSort } from "./questions-client";

export function QuestionsHeader({
  filter,
  setFilter,
  sort,
  setSort,
}: {
  filter: QuestionsFilter;
  setFilter: (filter: QuestionsFilter) => void;
  sort: QuestionsSort;
  setSort: (sort: QuestionsSort) => void;
}) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <FilterButton
        icon={SquareDot}
        label="Popular"
        isActive={filter === "popular"}
        onClick={() => setFilter("popular")}
      />
      <FilterButton
        icon={SquareDashed}
        label="Unanswered"
        isActive={filter === "unanswered"}
        onClick={() => setFilter("unanswered")}
      />
      <FilterButton
        icon={Square}
        label="All"
        isActive={filter === "all"}
        onClick={() => setFilter("all")}
      />
      <div className="ml-auto h-full flex flex-row">
        <SortDropdown sort={sort} setSort={setSort} />
        <NewQuestionButton />
      </div>
    </div>
  );
}

function FilterButton({
  icon: Icon,
  label,
  isActive,
  onClick,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  isActive: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      className={cn(
        "flex flex-row items-center h-full border-border border-r px-2 hover:bg-sidebar",
        isActive ? "bg-sidebar text-foreground" : "text-muted-foreground",
      )}
      onClick={onClick}
    >
      <Icon className="size-3 mr-1.5" />
      <span className="text-xs">{label}</span>
    </button>
  );
}

const SORT_LABELS: Record<QuestionsSort, string> = {
  "created-asc": "Created (asc.)",
  "created-desc": "Created (desc.)",
  "updated-asc": "Updated (asc.)",
  "updated-desc": "Updated (desc.)",
  "vote-asc": "Votes (asc.)",
  "vote-desc": "Votes (desc.)",
};

function SortDropdown({
  sort,
  setSort,
}: {
  sort: QuestionsSort;
  setSort: (sort: QuestionsSort) => void;
}) {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="flex flex-row h-full items-center border-border border-l px-2 text-xs text-muted-foreground min-w-32 w-32"
        >
          {SORT_LABELS[sort]}
          <ChevronDown className="size-3 ml-auto" />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="rounded-none min-w-32 p-0">
        {Object.entries(SORT_LABELS).map(([value, label]) => (
          <DropdownMenuItem
            key={value}
            onClick={() => setSort(value as QuestionsSort)}
            className="rounded-none px-2 py-1.5 text-xs cursor-pointer"
          >
            {label}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

function NewQuestionButton() {
  return (
    <Link
      type="button"
      className="flex flex-row h-full items-center px-2 border-border border-l bg-green-600 text-xs text-primary-foreground"
      href="questions/new"
    >
      <Plus className="size-3 mr-1.5" />
      Ask question
    </Link>
  );
}
