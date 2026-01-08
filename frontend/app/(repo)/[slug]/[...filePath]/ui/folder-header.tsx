export function FolderHeader({ folderPath }: { folderPath: string }) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b px-2 text-sm">
      {folderPath.split("/").pop()}/
    </div>
  );
}
