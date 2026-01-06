export function FileHeader({ filePath }: { filePath: string }) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b pl-2 text-sm font-mono">
      {`${filePath}`}
    </div>
  );
}
