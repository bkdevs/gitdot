import Link from "@/ui/link";

export default function Page() {
  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <div className="flex flex-col text-sm w-sm">
        <p>Import.</p>
        <p className="text-primary/60">
          Stuff about importing from GitHub
        </p>
        <Link href="/home">
          Finish
        </Link>
      </div>
    </div>
  );
}
