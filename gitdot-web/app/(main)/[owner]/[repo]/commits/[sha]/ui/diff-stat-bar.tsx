export function DiffStatBar({
  added,
  removed,
  linesPerBar = 5,
}: {
  added: number;
  removed: number;
  linesPerBar?: number;
}) {
  const total = added + removed;
  if (total === 0) return null;
  const hasAdded = added > 0;
  const hasRemoved = removed > 0;
  const minBars = hasAdded && hasRemoved ? 2 : 1;
  const barCount = Math.max(
    minBars,
    Math.min(Math.ceil(total / linesPerBar), 10),
  );

  let addedBars = Math.round((added / total) * barCount);
  if (hasAdded && addedBars < 1) addedBars = 1;
  if (hasRemoved && addedBars > barCount - 1) addedBars = barCount - 1;
  const removedBars = barCount - addedBars;

  return (
    <span className="font-mono select-none">
      <span className="text-green-600">{"+".repeat(addedBars)}</span>
      <span className="text-red-600">{"-".repeat(removedBars)}</span>
    </span>
  );
}
