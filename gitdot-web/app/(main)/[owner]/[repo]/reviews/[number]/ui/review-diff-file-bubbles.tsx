import { UserImage } from "@/(main)/[owner]/ui/user-image";

export function ReviewDiffFileBubbles({
  bubbleTop,
  userId,
}: {
  bubbleTop: number | null;
  userId: string | undefined;
}) {
  if (bubbleTop === null) return null;
  return (
    <div
      className="absolute z-50 flex flex-row items-center gap-1.5 left-full ml-2 px-2 py-0.5 bg-background animate-in fade-in duration-200"
      style={{ top: bubbleTop }}
    >
      <UserImage userId={userId} px={16} />
      <span className="text-xs font-sans text-muted-foreground">1</span>
    </div>
  );
}
