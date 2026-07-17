import { Activity, ShieldCheck } from "lucide-react";
import type { ReactNode } from "react";

import { Brand } from "@/components/brand";
import { MarketBoard } from "@/components/market-board";

type AuthShellProps = {
  eyebrow: string;
  title: string;
  description: string;
  children: ReactNode;
};

export function AuthShell({
  eyebrow,
  title,
  description,
  children,
}: AuthShellProps) {
  return (
    <main className="grid min-h-screen bg-[#0d0a10] lg:grid-cols-[minmax(0,1fr)_minmax(480px,0.82fr)]">
      <section className="flex min-h-screen flex-col px-5 py-6 sm:px-10 lg:px-16">
        <Brand />
        <div className="mx-auto my-auto w-full max-w-lg py-16">
          <p className="mb-5 text-xs font-semibold uppercase tracking-[0.22em] text-[#ff8175]">
            {eyebrow}
          </p>
          <h1 className="text-[32px] font-semibold leading-[1.08] tracking-[-0.04em] text-[#fff8f5] sm:whitespace-nowrap sm:text-4xl">
            {title}
          </h1>
          <p className="mb-8 mt-5 max-w-md leading-7 text-[#9b929f]">
            {description}
          </p>
          <div className="rounded-[28px] border border-[#302839] bg-[#121016] p-5 shadow-[0_24px_70px_rgba(0,0,0,0.22)] sm:p-7">
            {children}
          </div>
        </div>
        <div className="flex items-center gap-5 text-xs text-[#6f6775]">
          <span className="flex items-center gap-2">
            <ShieldCheck size={14} /> EdDSA secured
          </span>
          <span className="flex items-center gap-2">
            <Activity size={14} /> Session monitored
          </span>
        </div>
      </section>

      <aside className="m-3 hidden rounded-[38px] border border-[#2c2533] bg-[#141018] p-10 lg:flex lg:flex-col lg:justify-center">
        <div className="mx-auto w-full max-w-xl">
          <p className="mb-4 font-mono text-xs text-[#716878]">
            LIVE / MARKET WINDOW
          </p>
          <MarketBoard />
          <div className="mt-4 grid grid-cols-2 overflow-hidden rounded-2xl border border-[#302839] bg-[#19141e]">
            <div className="border-r border-[#302839] p-4">
              <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
                Custody mode
              </p>
              <p className="mt-2 text-sm text-[#ded6df]">Central ledger</p>
            </div>
            <div className="p-4">
              <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
                Settlement
              </p>
              <p className="mt-2 text-sm text-[#74ddbd]">Operational</p>
            </div>
          </div>
        </div>
      </aside>
    </main>
  );
}
