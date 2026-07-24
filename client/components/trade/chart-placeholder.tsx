"use client";

import { CandlestickChart } from "lucide-react";

export function ChartPlaceholder({ pair }: { pair: string }) {
  return (
    <div className="relative flex h-full min-h-[260px] flex-col overflow-hidden rounded-lg border border-[#2c2533] bg-[#100d14] sm:min-h-[300px] lg:min-h-0">
      <div className="flex items-center justify-between border-b border-[#2c2533] px-4 py-2.5">
        <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
          Chart · {pair}
        </p>
        <p className="text-[10px] uppercase tracking-[0.14em] text-[#57505e]">
          Candles soon
        </p>
      </div>
      <div className="relative flex flex-1 items-center justify-center">
        <div
          aria-hidden
          className="pointer-events-none absolute inset-0 opacity-40"
          style={{
            backgroundImage:
              "linear-gradient(rgba(48,40,57,0.55) 1px, transparent 1px), linear-gradient(90deg, rgba(48,40,57,0.55) 1px, transparent 1px)",
            backgroundSize: "48px 48px",
          }}
        />
        <div className="relative z-10 max-w-sm px-6 text-center">
          <div className="mx-auto mb-4 grid size-12 place-items-center rounded-2xl bg-[#271a20] text-[#ff8175]">
            <CandlestickChart size={22} />
          </div>
          <p className="text-sm font-medium text-[#fff8f5]">
            Price chart reserved
          </p>
        </div>
      </div>
    </div>
  );
}
