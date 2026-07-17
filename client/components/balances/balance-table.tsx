import { ArrowDownToLine, ArrowUpFromLine, LockKeyhole } from "lucide-react";

import type { AssetBalance } from "@/lib/types";

function displayAmount(value: number, decimals: number) {
  return new Intl.NumberFormat("en-US", {
    maximumFractionDigits: Math.min(decimals, 8),
  }).format(value / 10 ** decimals);
}

export function BalanceTable({ balances }: { balances: AssetBalance[] }) {
  if (balances.length === 0) {
    return (
      <div className="rounded-[28px] border border-dashed border-[#3a3142] bg-[#141018] px-6 py-20 text-center">
        <div className="mx-auto grid size-12 place-items-center rounded-2xl bg-[#271a20] text-[#ff8175]">
          <ArrowDownToLine size={20} />
        </div>
        <h2 className="mt-5 text-lg font-medium text-[#fff8f5]">
          Your ledger is empty
        </h2>
        <p className="mx-auto mt-2 max-w-sm text-sm leading-6 text-[#817884]">
          Assets will appear here after an admin demo credit or a supported
          deposit.
        </p>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto rounded-[28px] border border-[#302839] bg-[#141018]">
      <div className="min-w-[720px]">
        <div className="grid grid-cols-[1.4fr_1fr_1fr_1fr_auto] border-b border-[#302839] bg-[#19141e] px-6 py-4 text-[10px] uppercase tracking-[0.16em] text-[#716878]">
          <span>Asset</span>
          <span className="text-right">Available</span>
          <span className="text-right">Locked</span>
          <span className="text-right">Total</span>
          <span className="w-28 text-right">Actions</span>
        </div>

        {balances.map((balance) => {
          const total = balance.available + balance.locked;
          return (
            <div
              className="grid grid-cols-[1.4fr_1fr_1fr_1fr_auto] items-center border-[#292230] px-6 py-5 last:border-b-0 hover:bg-[#1b1621]"
              key={balance.asset_id}
            >
              <div className="flex items-center gap-3">
                <span className="grid size-10 place-items-center rounded-full bg-[#271a20] font-mono text-xs font-semibold text-[#ff8175]">
                  {balance.symbol.slice(0, 2)}
                </span>
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
                {balance.locked > 0 && <LockKeyhole size={13} />}
                {displayAmount(balance.locked, balance.decimals)}
              </p>
              <p className="text-right font-mono text-sm font-medium text-white">
                {displayAmount(total, balance.decimals)}
              </p>
              <div className="flex w-28 justify-end gap-2">
                <button
                  className="grid size-8 place-items-center rounded-full border border-[#3a3142] text-[#aaa1ad] hover:border-[#ff6f61] hover:text-[#ff8175]"
                  aria-label={`Deposit ${balance.symbol}`}
                >
                  <ArrowDownToLine size={14} />
                </button>
                <button
                  className="grid size-8 cursor-not-allowed place-items-center rounded-full border border-[#2d2734] text-[#514957]"
                  aria-label={`Withdraw ${balance.symbol}`}
                  disabled
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
