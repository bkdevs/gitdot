import { pluralize } from "@/util";

export function AnswersStripe({ count }: { count: number }) {
  return (
    <div className="flex flex-row max-w-4xl items-center text-sm text-muted-foreground">
      {count > 0 && (
        <>
          <span className="ml-2">{pluralize(count, "answer")}</span>
          <span className="ml-1">sorted by:</span>
          <select className="bg-transparent border border-transparent underline rounded px-0 py-0.5">
            <option>highest score</option>
            <option>date modified</option>
            <option>date created</option>
          </select>
        </>
      )}
    </div>
  );
}
