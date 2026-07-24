"use client";

import { LogOut, Menu, WalletCards, X } from "lucide-react";
import Link from "next/link";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";

import { Brand } from "@/components/brand";
import { TradeNavLink } from "@/components/trade-nav-link";
import { Button } from "@/components/ui/button";
import { logout, restoreSession } from "@/lib/api";
import { getSession } from "@/lib/session";
import type { Session } from "@/lib/types";

export function SiteHeader({
  variant = "default",
}: {
  variant?: "default" | "desk";
}) {
  const [session, setSession] = useState<Session | null>(null);
  const [open, setOpen] = useState(false);
  const router = useRouter();
  const desk = variant === "desk";

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

  async function handleLogout() {
    await logout();
    router.push("/");
  }

  return (
    <header
      className={
        desk
          ? "sticky top-0 z-40 shrink-0 border-b border-[#2c2533] bg-[#100d14]/95 backdrop-blur"
          : "sticky top-3 z-40 px-3"
      }
    >
      <div
        className={
          desk
            ? "flex h-12 w-full items-center justify-between px-3 sm:px-4"
            : "mx-auto flex h-16 max-w-[1380px] items-center justify-between rounded-2xl border border-[#2c2533] bg-[#100d14]/95 px-5 shadow-[0_16px_50px_rgba(0,0,0,0.28)] backdrop-blur lg:px-6"
        }
      >
        <Brand />

        <nav
          className={`hidden items-center gap-1 text-sm text-[#aaa1ad] md:flex ${
            desk ? "" : "rounded-full bg-[#17131d] p-1"
          }`}
        >
          <Link
            className="rounded-full px-3 py-1.5 hover:bg-[#241d2b] hover:text-white sm:px-4 sm:py-2"
            href="/markets"
          >
            Markets
          </Link>
          <Link
            className="rounded-full px-3 py-1.5 hover:bg-[#241d2b] hover:text-white sm:px-4 sm:py-2"
            href="/balances"
          >
            Balances
          </Link>
          <TradeNavLink className="rounded-full px-3 py-1.5 hover:bg-[#241d2b] hover:text-white sm:px-4 sm:py-2">
            Trade
          </TradeNavLink>
        </nav>

        <div className="hidden items-center gap-2 md:flex">
          {session ? (
            <>
              <Link href="/balances">
                <Button tone="quiet">
                  <WalletCards size={16} />
                  {session.user.name.split(" ")[0]}
                </Button>
              </Link>
              <Button tone="quiet" onClick={handleLogout} aria-label="Log out">
                <LogOut size={16} />
              </Button>
            </>
          ) : (
            <>
              <Link href="/login">
                <Button tone="quiet">Log in</Button>
              </Link>
              <Link href="/signup">
                <Button>Open account</Button>
              </Link>
            </>
          )}
        </div>

        <button className="text-white md:hidden" onClick={() => setOpen(!open)}>
          {open ? <X /> : <Menu />}
        </button>
      </div>

      {open && (
        <div
          className={
            desk
              ? "border-t border-[#2c2533] bg-[#15111a] p-4 md:hidden"
              : "mx-auto mt-2 max-w-[1380px] rounded-2xl border border-[#2c2533] bg-[#15111a] p-5 shadow-2xl md:hidden"
          }
        >
          <div className="flex flex-col gap-4 text-sm text-[#ded6df]">
            <Link href="/markets" onClick={() => setOpen(false)}>
              Markets
            </Link>
            <Link href="/balances" onClick={() => setOpen(false)}>
              Balances
            </Link>
            <TradeNavLink onClick={() => setOpen(false)}>Trade</TradeNavLink>
            {session ? (
              <Button tone="quiet" onClick={handleLogout}>
                Log out
              </Button>
            ) : (
              <div className="grid grid-cols-2 gap-2">
                <Link href="/login">
                  <Button className="w-full" tone="quiet">
                    Log in
                  </Button>
                </Link>
                <Link href="/signup">
                  <Button className="w-full">Sign up</Button>
                </Link>
              </div>
            )}
          </div>
        </div>
      )}
    </header>
  );
}
