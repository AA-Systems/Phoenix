import { displayDelta } from "@/lib/balances";
import type { LedgerEntry } from "@/lib/types";

function labelFor(entryType: LedgerEntry["entry_type"]) {
  switch (entryType) {
    case "deposit":
      return "Deposit";
    case "withdrawal":
      return "Withdrawal";
    case "lock":
      return "Lock";
    case "unlock":
      return "Unlock";
    case "trade":
      return "Trade";
    case "fee":
      return "Fee";
    case "adjustment":
      return "Adjustment";
  }
}

function toneFor(entryType: LedgerEntry["entry_type"]) {
  switch (entryType) {
    case "deposit":
      return "bg-[#143028] text-[#74ddbd] ring-[#1f4a3c]";
    case "lock":
      return "bg-[#321f26] text-[#ff8175] ring-[#4a3037]";
    case "unlock":
      return "bg-[#1f2432] text-[#9db7ff] ring-[#2f3a52]";
    case "trade":
      return "bg-[#2a2430] text-[#e6dfe7] ring-[#3a3142]";
    case "withdrawal":
      return "bg-[#321818] text-[#ff9e96] ring-[#4a2a2a]";
    case "fee":
      return "bg-[#2a2418] text-[#e6c27a] ring-[#3d3420]";
    case "adjustment":
      return "bg-[#241c28] text-[#cfc6d3] ring-[#3a3142]";
  }
}

export function LedgerActivity({ entries }: { entries: LedgerEntry[] }) {
  if (entries.length === 0) {
    return (
      <div className="rounded-[28px] border border-dashed border-[#3a3142] bg-[#141018]/80 px-6 py-16 text-center">
        <p className="text-sm text-[#817884]">
          No ledger activity yet. Credits and trades will show up here.
        </p>
      </div>
    );
  }

  return (
    <div className="relative overflow-hidden rounded-[28px] border border-[#302839] bg-[#141018]/90">
      <div className="absolute bottom-6 left-[27px] top-6 w-px bg-linear-to-b from-[#ff6f61]/50 via-[#3a3142] to-transparent" />
      <ul className="divide-y divide-[#292230]">
        {entries.map((entry, index) => (
          <li
            className="page-reveal relative grid grid-cols-[56px_1fr_auto] items-center gap-3 px-4 py-4 sm:px-6"
            key={entry.id}
            style={{ animationDelay: `${60 + index * 40}ms` }}
          >
            <div className="relative grid place-items-center">
              <span className="z-10 size-2.5 rounded-full bg-[#ff6f61] shadow-[0_0_12px_#ff6f61]" />
            </div>
            <div className="min-w-0">
              <div className="flex flex-wrap items-center gap-2">
                <span
                  className={`rounded-full px-2.5 py-0.5 text-[10px] font-semibold uppercase tracking-[0.12em] ring-1 ${toneFor(entry.entry_type)}`}
                >
                  {labelFor(entry.entry_type)}
                </span>
                <span className="font-mono text-sm text-[#ddd5df]">
                  {entry.asset_symbol}
                </span>
              </div>
              <p className="mt-1.5 text-xs text-[#716878]">
                {new Date(entry.created_at).toLocaleString(undefined, {
                  dateStyle: "medium",
                  timeStyle: "short",
                })}
              </p>
            </div>
            <div className="text-right">
              <p
                className={`font-mono text-sm ${
                  entry.available_delta >= 0
                    ? "text-[#74ddbd]"
                    : "text-[#ff9e96]"
                }`}
              >
                {displayDelta(entry.available_delta, entry.asset_decimals)}
              </p>
              {entry.locked_delta !== 0 ? (
                <p className="mt-1 font-mono text-[11px] text-[#928997]">
                  locked{" "}
                  {displayDelta(entry.locked_delta, entry.asset_decimals)}
                </p>
              ) : null}
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}
