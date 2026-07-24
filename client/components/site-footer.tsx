"use client";

import { ArrowUpRight, Flame } from "lucide-react";
import Link from "next/link";

import { TradeNavLink } from "@/components/trade-nav-link";

const footerClass =
  "group flex min-h-24 flex-col justify-between border-b border-[#302839] p-4 text-sm text-[#a198a5] transition-colors hover:bg-[#141018] hover:text-[#fff8f5] sm:border-b-0 sm:border-l";

export function SiteFooter() {
  return (
    <footer className="relative mt-24 overflow-hidden border-t border-[#302839] bg-[#0b090d]">
      <div className="relative z-10 mx-auto flex max-w-[1380px] flex-col justify-between gap-14 px-6 pb-20 pt-20 sm:px-10 lg:flex-row lg:items-start lg:px-8 lg:pb-24 lg:pt-24">
        <div>
          <Link
            className="inline-flex items-center gap-3"
            href="/"
            aria-label="Pheonix home"
          >
            <span className="grid size-10 place-items-center rounded-full bg-[#ff6f61] text-[#140d12]">
              <Flame fill="currentColor" size={20} strokeWidth={1.7} />
            </span>
            <span className="text-sm font-bold tracking-[0.18em] text-[#fff8f5]">
              PHEONIX
            </span>
          </Link>
          <p className="mt-6 max-w-sm text-base leading-7 text-[#817787]">
            A quieter place to read the market,
            <br />
            hold your position, and move.
          </p>
        </div>

        <nav
          aria-label="Footer navigation"
          className="grid w-full max-w-xl grid-cols-2 border-t border-[#302839] sm:grid-cols-5 lg:border-t-0"
        >
          <FooterCell href="/markets" index={1} label="Markets" />
          <TradeNavLink className={footerClass}>
            <span className="font-mono text-[9px] text-[#554d59]">02</span>
            <span className="flex items-center justify-between gap-2">
              Trade
              <ArrowUpRight
                className="text-[#625a67] transition-transform group-hover:-translate-y-0.5 group-hover:translate-x-0.5 group-hover:text-[#ff8175]"
                size={14}
              />
            </span>
          </TradeNavLink>
          <FooterCell href="/balances" index={3} label="Balances" />
          <FooterCell href="/login" index={4} label="Log in" />
          <FooterCell href="/signup" index={5} label="Open account" />
        </nav>
      </div>

      <div className="relative min-h-[380px] overflow-hidden border-t border-[#241f2a]">
        <div className="absolute left-1/2 top-14 z-10 grid size-14 -translate-x-1/2 place-items-center rounded-full border border-[#6b3b3d] bg-[#1b1117] text-[#ff8175] shadow-[0_0_50px_rgba(255,111,97,0.3)]">
          <Flame
            className="phoenix-flame"
            fill="currentColor"
            size={27}
            strokeWidth={1.3}
          />
        </div>

        <span className="phoenix-ember absolute left-[18%] top-[65%] size-1.5 rounded-full bg-[#ff8175] shadow-[0_0_14px_#ff6f61]" />
        <span className="phoenix-ember absolute right-[22%] top-[72%] size-1 rounded-full bg-[#ffb45e] shadow-[0_0_14px_#ffb45e] [animation-delay:-2.8s]" />
        <span className="phoenix-ember absolute left-[62%] top-[58%] size-1 rounded-full bg-[#ff8175] shadow-[0_0_12px_#ff6f61] [animation-delay:-4s]" />

        <div className="phoenix-footer-glow absolute -bottom-40 left-1/2 h-80 w-[92%] rounded-[50%] bg-[#ff6f61] opacity-75 blur-[100px]" />
        <div className="phoenix-footer-glow absolute -bottom-36 left-1/2 h-52 w-[50%] rounded-[50%] bg-[#ffc060] opacity-65 blur-[70px] [animation-delay:-2s]" />

        <p className="pointer-events-none absolute bottom-2 left-1/2 -translate-x-1/2 whitespace-nowrap text-[clamp(6rem,19vw,18rem)] font-black leading-[0.7] tracking-[-0.085em] text-[#ffd0c8]/25">
          PHEONIX
        </p>

        <div className="absolute bottom-5 left-0 right-0 z-10 mx-auto flex max-w-[1380px] flex-col gap-3 px-6 font-mono text-[9px] uppercase tracking-[0.14em] text-[#35171a] sm:flex-row sm:items-center sm:justify-between sm:px-10 lg:px-8">
          <span>© 2026 PHEONIX Exchange</span>
          <span>Spot markets · Devnet ready</span>
        </div>
      </div>
    </footer>
  );
}

function FooterCell({
  href,
  index,
  label,
}: {
  href: string;
  index: number;
  label: string;
}) {
  return (
    <Link className={footerClass} href={href}>
      <span className="font-mono text-[9px] text-[#554d59]">0{index}</span>
      <span className="flex items-center justify-between gap-2">
        {label}
        <ArrowUpRight
          className="text-[#625a67] transition-transform group-hover:-translate-y-0.5 group-hover:translate-x-0.5 group-hover:text-[#ff8175]"
          size={14}
        />
      </span>
    </Link>
  );
}
