import { ArrowDownToLine, ArrowUpFromLine, LockKeyhole } from "lucide-react";

import { AssetIcon } from "@/components/markets/asset-icon";
import { displayAmount } from "@/lib/balances";
import type { AssetBalance } from "@/lib/types";

export function BalanceTable({
  balances,
  onDemoCredit,
  crediting = false,
}: {
  balances: AssetBalance[];
  onDemoCredit?: (symbol: string) => void;
  crediting?: boolean;
}) {
  if (balances.length === 0) {
    return (
      <div className="rounded-[28px] border border-dashed border-[#3a3142] bg-[#141018]/80 px-6 py-20 text-center">
        <div className="mx-auto grid size-12 place-items-center rounded-2xl bg-[#271a20] text-[#ff8175]">
          <ArrowDownToLine size={20} />
        </div>
        <h2 className="mt-5 text-lg font-medium text-[#fff8f5]">
          Your ledger is empty
        </h2>
        <p className="mx-auto mt-2 max-w-sm text-sm leading-6 text-[#817884]">
          Use <span className="text-[#ff8175]">Get test funds</span> above to
          credit a demo faucet pack, then start trading.
        </p>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto rounded-[28px] border border-[#302839] bg-[#141018]/90 backdrop-blur-sm">
      <div className="min-w-[760px]">
        <div className="grid grid-cols-[1.5fr_1fr_1fr_1.15fr_auto] border-b border-[#302839] bg-[#19141e]/90 px-6 py-4 text-[10px] uppercase tracking-[0.16em] text-[#716878]">
          <span>Asset</span>
          <span className="text-right">Available</span>
          <span className="text-right">Locked</span>
          <span className="text-right">Composition</span>
          <span className="w-28 text-right">Actions</span>
        </div>

        {balances.map((balance, index) => {
          const total = balance.available + balance.locked;
          const availablePct =
            total > 0 ? (balance.available / total) * 100 : 100;
          const lockedPct = total > 0 ? (balance.locked / total) * 100 : 0;

          return (
            <div
              className="page-reveal grid grid-cols-[1.5fr_1fr_1fr_1.15fr_auto] items-center border-b border-[#292230] px-6 py-5 last:border-b-0 hover:bg-[#1b1621]/80"
              key={balance.asset_id}
              style={{ animationDelay: `${80 + index * 45}ms` }}
            >
              <div className="flex items-center gap-3">
                <AssetIcon size={40} symbol={balance.symbol} />
                <div>
                  <p className="text-sm font-semibold text-[#fff8f5]">
                    {balance.symbol}
                  </p>
                  <p className="mt-0.5 text-xs text-[#716878]">
                    {balance.name}
                  </p>
                </div>
              </div>
              <p className="text-right font-mono text-sm text-[#ddd5df]">
                {displayAmount(balance.available, balance.decimals)}
              </p>
              <p className="flex items-center justify-end gap-2 font-mono text-sm text-[#928997]">
                {balance.locked > 0 && (
                  <LockKeyhole className="text-[#ff8175]" size={13} />
                )}
                {displayAmount(balance.locked, balance.decimals)}
              </p>
              <div className="flex flex-col items-end gap-1.5">
                <p className="font-mono text-sm font-medium text-white">
                  {displayAmount(total, balance.decimals)}
                </p>
                <div className="flex h-1.5 w-28 overflow-hidden rounded-full bg-[#1d1722]">
                  <span
                    className="bar-grow h-full bg-[#74ddbd]"
                    style={{
                      width: `${availablePct}%`,
                      animationDelay: `${120 + index * 45}ms`,
                    }}
                  />
                  <span
                    className="bar-grow h-full bg-[#ff6f61]"
                    style={{
                      width: `${lockedPct}%`,
                      animationDelay: `${150 + index * 45}ms`,
                    }}
                  />
                </div>
              </div>
              <div className="flex w-28 justify-end gap-2">
                <button
                  className="grid size-8 place-items-center rounded-full border border-[#3a3142] text-[#aaa1ad] transition hover:border-[#ff6f61] hover:text-[#ff8175] disabled:opacity-50"
                  aria-label={`Credit demo ${balance.symbol}`}
                  disabled={crediting || !onDemoCredit}
                  onClick={() => onDemoCredit?.(balance.symbol)}
                  type="button"
                >
                  <ArrowDownToLine size={14} />
                </button>
                <button
                  className="grid size-8 cursor-not-allowed place-items-center rounded-full border border-[#2d2734] text-[#514957]"
                  aria-label={`Withdraw ${balance.symbol}`}
                  disabled
                  type="button"
                >
                  <ArrowUpFromLine size={14} />
                </button>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
