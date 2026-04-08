import Image from "next/image";

export function UserImage({
  user,
  px = 32,
}: {
  user: { name: string; image?: string | null };
  px?: number;
}) {
  const src =
    user.name === "baepaul"
      ? "/paul-penguin.jpeg"
      : user.image
        ? `data:image/webp;base64,${user.image}`
        : null;

  return src ? (
    <Image
      src={src}
      alt={user.name}
      width={px}
      height={px}
      unoptimized
      className="rounded-full"
      style={{ width: px, height: px }}
    />
  ) : (
    <div
      className="rounded-full bg-foreground flex items-center justify-center shrink-0"
      style={{ width: px, height: px }}
    >
      <span
        className={`font-mono font-light text-background leading-none ${px <= 20 ? "text-xs" : "text-sm"}`}
      >
        {user.name[0].toUpperCase()}
      </span>
    </div>
  );
}
