"use client";

import { ArrowRight, CandlestickChart, WalletCards } from "lucide-react";
import Link from "next/link";
import { useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import { restoreSession } from "@/lib/api";
import { getSession } from "@/lib/session";
import type { Session } from "@/lib/types";

export function HeroActions() {
  const [session, setSession] = useState<Session | null | undefined>(undefined);

  useEffect(() => {
    const sync = () => setSession(getSession());
    sync();
    let active = true;
    restoreSession().then((restored) => {
      if (active) setSession(restored);
    });
    window.addEventListener("cex-session", sync);
    return () => {
      active = false;
      window.removeEventListener("cex-session", sync);
    };
  }, []);

  if (session === undefined) {
    return <div className="mt-10 h-12" aria-hidden />;
  }

  if (session) {
    return (
      <div className="mt-10 flex flex-wrap gap-3">
        <Link href="/markets">
          <Button className="h-12 px-6">
            Browse markets <CandlestickChart size={17} />
          </Button>
        </Link>
        <Link href="/balances">
          <Button className="h-12 px-6" tone="quiet">
            <WalletCards size={16} />
            Balances
          </Button>
        </Link>
      </div>
    );
  }

  return (
    <div className="mt-10 flex flex-wrap gap-3">
      <Link href="/signup">
        <Button className="h-12 px-6">
          Create your ledger <ArrowRight size={17} />
        </Button>
      </Link>
      <Link href="/login">
        <Button className="h-12 px-6" tone="quiet">
          Log in
        </Button>
      </Link>
    </div>
  );
}
