import Image from "next/image";

export function UserProfile({ owner }: { owner: string }) {
  return (
    <div className="flex flex-col items-end">
      <Image
      src="/paul-penguin.jpeg"
      alt={owner}
      width={32}
      height={32}
      className="rounded-full"
      />
      <p className="font-semibold text-sm">{owner}</p>
      <p className="text-sm text-muted-foreground">paul@gitdot.io</p>
      <p className="text-sm text-muted-foreground">brooklyn, ny</p>
    </div>
  );
}
