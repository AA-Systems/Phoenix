import { ArrowDownRight, ArrowUpRight } from "lucide-react";

const markets = [
  { pair: "SOL / USDC", price: "149.82", change: "+4.12%", up: true },
  { pair: "SOL / USD", price: "149.76", change: "+4.08%", up: true },
  { pair: "SOL / INR", price: "12,504.10", change: "-0.34%", up: false },
];

export function MarketBoard() {
  return (
    <div className="overflow-hidden rounded-[28px] border border-[#302839] bg-[#15111a] shadow-[0_28px_80px_rgba(0,0,0,0.3)]">
      <div className="flex items-center justify-between border-b border-[#2c2533] px-6 py-5">
        <div>
          <p className="text-xs uppercase tracking-[0.18em] text-[#817787]">
            Market pulse
          </p>
          <p className="mt-1 text-sm text-[#f0e9f0]">Demo session</p>
        </div>
        <span className="flex items-center gap-2 rounded-full bg-[#13211e] px-3 py-1.5 text-xs text-[#74ddbd]">
          <span className="size-1.5 rounded-full bg-[#74ddbd]" />
          Engine online
        </span>
      </div>

      <div className="grid grid-cols-[1fr_auto_auto] px-6 py-3 text-[10px] uppercase tracking-[0.16em] text-[#716878]">
        <span>Market</span>
        <span className="text-right">Last</span>
        <span className="w-24 text-right">24h</span>
      </div>

      {markets.map((market) => (
        <div
          className="grid grid-cols-[1fr_auto_auto] items-center border-t border-[#292230] px-6 py-5 transition-colors hover:bg-[#1b1621]"
          key={market.pair}
        >
          <div className="flex items-center gap-3">
            <span className="grid size-9 place-items-center rounded-full bg-[#251b26] font-mono text-[10px] text-[#ff8175]">
              {market.pair.slice(0, 2)}
            </span>
            <span className="text-sm font-medium text-[#fff8f5]">
              {market.pair}
            </span>
          </div>
          <span className="font-mono text-sm text-[#dcd4de]">
            {market.price}
          </span>
          <span
            className={`flex w-24 items-center justify-end gap-1 font-mono text-xs ${market.up ? "text-[#74ddbd]" : "text-[#ff8175]"}`}
          >
            {market.up ? (
              <ArrowUpRight size={14} />
            ) : (
              <ArrowDownRight size={14} />
            )}
            {market.change}
          </span>
        </div>
      ))}
    </div>
  );
}
