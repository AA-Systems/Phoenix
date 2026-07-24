import Link from "next/link";
import { ArrowUpRight, CandlestickChart } from "lucide-react";

import { MarketPairIcons } from "@/components/markets/asset-icon";
import { formatMarketPair, splitMarketSymbol } from "@/lib/markets";
import type { Market, MarketStatus } from "@/lib/types";

const statusTone: Record<MarketStatus, string> = {
  trading: "bg-[#13211e] text-[#74ddbd]",
  halted: "bg-[#2a2214] text-[#e2c07a]",
  archived: "bg-[#221f26] text-[#716878]",
};

export function MarketsTable({ markets }: { markets: Market[] }) {
  if (markets.length === 0) {
    return (
      <div className="rounded-[28px] border border-dashed border-[#3a3142] bg-[#141018]/80 px-6 py-20 text-center">
        <div className="mx-auto grid size-12 place-items-center rounded-2xl bg-[#271a20] text-[#ff8175]">
          <CandlestickChart size={20} />
        </div>
        <h2 className="mt-5 text-lg font-medium text-[#fff8f5]">
          No markets listed
        </h2>
        <p className="mx-auto mt-2 max-w-sm text-sm leading-6 text-[#817884]">
          Seed the exchange or add a market from the admin API to see pairs
          here.
        </p>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto rounded-[28px] border border-[#302839] bg-[#141018]/90 backdrop-blur-sm">
      <div className="min-w-[820px]">
        <div className="grid grid-cols-[1.6fr_0.9fr_1fr_1fr_1fr_auto] border-b border-[#302839] bg-[#19141e]/90 px-6 py-4 text-[10px] uppercase tracking-[0.16em] text-[#716878]">
          <span>Market</span>
          <span>Status</span>
          <span className="text-right">Tick</span>
          <span className="text-right">Step</span>
          <span className="text-right">Min qty</span>
          <span className="w-24 text-right">Trade</span>
        </div>

        {markets.map((market, index) => {
          const [base, quote] = splitMarketSymbol(market.symbol);

          return (
            <div
              className="page-reveal grid grid-cols-[1.6fr_0.9fr_1fr_1fr_1fr_auto] items-center border-b border-[#292230] px-6 py-5 last:border-b-0 hover:bg-[#1b1621]/80"
              key={market.id}
              style={{ animationDelay: `${80 + index * 45}ms` }}
            >
              <div className="flex items-center gap-3">
                <MarketPairIcons base={base} quote={quote} />
                <div>
                  <p className="text-sm font-semibold text-[#fff8f5]">
                    {formatMarketPair(market.symbol)}
                  </p>
                  <p className="mt-0.5 text-xs text-[#716878]">{market.name}</p>
                </div>
              </div>

              <span
                className={`inline-flex w-fit items-center rounded-full px-2.5 py-1 text-[10px] uppercase tracking-[0.14em] ${statusTone[market.status]}`}
              >
                {market.status}
              </span>

              <p className="text-right font-mono text-sm text-[#ddd5df]">
                {market.price_tick_size}
              </p>
              <p className="text-right font-mono text-sm text-[#ddd5df]">
                {market.quantity_step_size}
              </p>
              <p className="text-right font-mono text-sm text-[#ddd5df]">
                {market.min_order_quantity}
              </p>

              <div className="flex w-24 justify-end">
                {market.status === "trading" ? (
                  <Link
                    className="inline-flex items-center gap-1 rounded-full border border-[#3a3142] bg-[#1a151f] px-3 py-1.5 text-xs text-[#e6dfe7] transition-colors hover:border-[#ff6f61] hover:text-[#fff8f5]"
                    href={`/trade/${encodeURIComponent(market.symbol)}`}
                  >
                    Trade
                    <ArrowUpRight size={13} />
                  </Link>
                ) : (
                  <span className="px-3 py-1.5 text-xs text-[#57505e]">—</span>
                )}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
