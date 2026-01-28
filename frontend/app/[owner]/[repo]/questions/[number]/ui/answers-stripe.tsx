import { pluralize } from "@/util";

export function AnswersStripe({ count }: { count: number }) {
  return (
    <div className="flex justify-between items-center">
      <span className="text-lg font-medium">{pluralize(count, "Answer")}</span>
      <div className="flex items-center gap-2 text-sm">
        <span className="text-muted-foreground">Sorted by:</span>
        <select className="bg-transparent border border-border rounded px-2 py-1">
          <option>Highest score</option>
          <option>Date modified</option>
          <option>Date created</option>
        </select>
      </div>
    </div>
  );
}
