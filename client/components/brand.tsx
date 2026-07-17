import { Flame } from "lucide-react";
import Link from "next/link";

export function Brand() {
  return (
    <Link
      href="/"
      className="flex items-center gap-3"
      aria-label="Pheonix home"
    >
      <span className="grid size-9 place-items-center rounded-full bg-[#ff6f61] text-[#140d12]">
        <Flame size={19} fill="currentColor" strokeWidth={1.8} />
      </span>
      <span className="text-sm font-bold tracking-[0.16em] text-[#fff8f5]">
        PHEON<span className="text-[#ff6f61]">IX</span>
      </span>
    </Link>
  );
}
