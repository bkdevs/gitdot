import { User } from "lucide-react";
import Image from "next/image";

export function UserImage({
  image,
  name,
}: {
  image?: string | null;
  name?: string;
}) {
  return image ? (
    <Image
      src={`data:image/webp;base64,${image}`}
      alt={name ?? "Profile"}
      width={32}
      height={32}
      unoptimized
      className="rounded-full"
    />
  ) : (
    <User className="size-8" />
  );
}
