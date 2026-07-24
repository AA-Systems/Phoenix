"use client";

import Link from "next/link";
import { useEffect, useState, type ReactNode } from "react";

import { listMarkets } from "@/lib/api";

/** Resolves Trade href from the first trading market in the DB (no hardcoded pair). */
export function useTradeHref(): string {
  const [href, setHref] = useState("/markets");

  useEffect(() => {
    let active = true;
    listMarkets()
      .then((markets) => {
        if (!active) return;
        const first =
          markets.find((market) => market.status === "trading") ?? markets[0];
        if (first) {
          setHref(`/trade/${encodeURIComponent(first.symbol)}`);
        }
      })
      .catch(() => {
        /* keep /markets fallback */
      });
    return () => {
      active = false;
    };
  }, []);

  return href;
}

export function TradeNavLink({
  className,
  children,
  onClick,
}: {
  className?: string;
  children: ReactNode;
  onClick?: () => void;
}) {
  const href = useTradeHref();
  return (
    <Link className={className} href={href} onClick={onClick}>
      {children}
    </Link>
  );
}
