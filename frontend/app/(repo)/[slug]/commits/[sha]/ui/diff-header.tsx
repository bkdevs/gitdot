export function DiffHeader({ path }: { path: string }) {
  return (
    <div className="flex flex-row w-full h-9 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
      {path}
    </div>
  );
}
