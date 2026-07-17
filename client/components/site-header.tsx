"use client";

import { LogOut, Menu, WalletCards, X } from "lucide-react";
import Link from "next/link";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";

import { Brand } from "@/components/brand";
import { Button } from "@/components/ui/button";
import { logout } from "@/lib/api";
import { getSession } from "@/lib/session";
import type { Session } from "@/lib/types";

export function SiteHeader() {
  const [session, setSession] = useState<Session | null>(null);
  const [open, setOpen] = useState(false);
  const router = useRouter();

  useEffect(() => {
    const sync = () => setSession(getSession());
    sync();
    window.addEventListener("cex-session", sync);
    return () => window.removeEventListener("cex-session", sync);
  }, []);

  async function handleLogout() {
    await logout();
    router.push("/");
  }

  return (
    <header className="sticky top-3 z-40 px-3">
      <div className="mx-auto flex h-16 max-w-[1380px] items-center justify-between rounded-2xl border border-[#2c2533] bg-[#100d14]/95 px-5 shadow-[0_16px_50px_rgba(0,0,0,0.28)] backdrop-blur lg:px-6">
        <Brand />

        <nav className="hidden items-center gap-1 rounded-full bg-[#17131d] p-1 text-sm text-[#aaa1ad] md:flex">
          <Link
            className="rounded-full px-4 py-2 hover:bg-[#241d2b] hover:text-white"
            href="/#markets"
          >
            Markets
          </Link>
          <Link
            className="rounded-full px-4 py-2 hover:bg-[#241d2b] hover:text-white"
            href="/balances"
          >
            Balances
          </Link>
          <span className="cursor-not-allowed rounded-full px-4 py-2 text-[#57505e]">
            Trade · soon
          </span>
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
        <div className="mx-auto mt-2 max-w-[1380px] rounded-2xl border border-[#2c2533] bg-[#15111a] p-5 shadow-2xl md:hidden">
          <div className="flex flex-col gap-4 text-sm text-[#ded6df]">
            <Link href="/#markets" onClick={() => setOpen(false)}>
              Markets
            </Link>
            <Link href="/balances" onClick={() => setOpen(false)}>
              Balances
            </Link>
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
