import { cn } from "@/util";

export function QuestionMark({ className }: { className?: string }) {
  return (
    <span
      className={cn(
        className,
        "inline-flex items-center justify-center text-[15px] leading-none",
      )}
    >
      ?
    </span>
  );
}

/**
 * right angle triangle pointing up
 */
export function TriangleUp({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      width="16.971"
      height="12"
      viewBox="0 0 16.971 12"
    >
      <title>Up</title>
      <polygon points="8.485,0 0,12 16.971,12" fill="currentColor" />
    </svg>
  );
}

/**
 * right angle triangle pointing down
 */
export function TriangleDown({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      width="16.971"
      height="12"
      viewBox="0 0 16.971 12"
    >
      <title>Down</title>
      <polygon points="0,0 16.971,0 8.485,12" fill="currentColor" />
    </svg>
  );
}
