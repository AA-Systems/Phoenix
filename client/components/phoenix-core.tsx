import { Flame } from "lucide-react";

const embers = [
  "left-[14%] top-[72%] [animation-delay:-1.2s]",
  "left-[28%] top-[48%] [animation-delay:-3.8s]",
  "right-[22%] top-[66%] [animation-delay:-2.4s]",
  "right-[10%] top-[38%] [animation-delay:-4.7s]",
  "left-[44%] top-[82%] [animation-delay:-0.6s]",
];

export function PhoenixCore() {
  return (
    <div
      aria-hidden="true"
      className="pointer-events-none absolute inset-0 overflow-hidden"
    >
      <div className="phoenix-core absolute right-[-8%] top-[-10%] aspect-square w-[78%] max-w-[520px]">
        <div className="phoenix-ring absolute inset-0 rounded-full border border-[#3b2932]" />
        <div className="phoenix-ring-reverse absolute inset-[13%] rounded-full border border-dashed border-[#4a3037]" />
        <div className="absolute inset-[28%] grid place-items-center rounded-full border border-[#5a3339] bg-[#1b1117] shadow-[0_0_80px_rgba(255,111,97,0.12)]">
          <Flame
            className="phoenix-flame text-[#ff6f61]"
            fill="currentColor"
            size={82}
            strokeWidth={1.2}
          />
        </div>
        <span className="absolute left-[7%] top-1/2 size-2 rounded-full bg-[#ff6f61] shadow-[0_0_18px_#ff6f61]" />
        <span className="absolute right-[18%] top-[12%] size-1.5 rounded-full bg-[#74ddbd] shadow-[0_0_14px_#74ddbd]" />
      </div>

      {embers.map((position) => (
        <span
          className={`phoenix-ember absolute size-1.5 rounded-full bg-[#ff8175] shadow-[0_0_12px_#ff6f61] ${position}`}
          key={position}
        />
      ))}
    </div>
  );
}
