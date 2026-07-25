"use client";

import Link from "next/link";
import { useEffect, useMemo, useState } from "react";

import { AssetIcon } from "@/components/markets/asset-icon";
import { useAssetDecimals } from "@/components/trade/asset-decimals";
import { Button } from "@/components/ui/button";
import { createOrder } from "@/lib/api";
import { displayAmount } from "@/lib/balances";
import {
  formatPrice,
  marketDecimals,
  parseHumanToRaw,
  rawToHumanInput,
} from "@/lib/trade-format";
import type { AssetBalance, Market, OrderType } from "@/lib/types";

const PCT_STEPS = [0, 25, 50, 75, 100] as const;

export function OrderForm({
  market,
  balances,
  priceHint,
  priceHintNonce = 0,
  onSubmitted,
}: {
  market: Market;
  balances: AssetBalance[] | null;
  priceHint: number | null;
  priceHintNonce?: number;
  onSubmitted?: () => void;
}) {
  const [side, setSide] = useState<OrderType>("buy");
  const [price, setPrice] = useState("");
  const [quantity, setQuantity] = useState("");
  const [pct, setPct] = useState(0);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState("");
  const [ok, setOk] = useState("");
  const decimalsBySymbol = useAssetDecimals();

  const { base, quote, baseDecimals, quoteDecimals } = marketDecimals(
    market.symbol,
    decimalsBySymbol,
  );

  const baseBal = balances?.find((b) => b.symbol === base);
  const quoteBal = balances?.find((b) => b.symbol === quote);
  const availableRaw =
    side === "buy" ? (quoteBal?.available ?? 0) : (baseBal?.available ?? 0);
  const availableLabel =
    side === "buy"
      ? quoteBal
        ? `${displayAmount(quoteBal.available, quoteBal.decimals)} ${quote}`
        : `— ${quote}`
      : baseBal
        ? `${displayAmount(baseBal.available, baseBal.decimals)} ${base}`
        : `— ${base}`;

  const orderValueLabel = useMemo(() => {
    const priceRaw = parseHumanToRaw(price, quoteDecimals);
    const qtyRaw = parseHumanToRaw(quantity, baseDecimals);
    if (priceRaw === null || qtyRaw === null) return null;
    const notional =
      (BigInt(priceRaw) * BigInt(qtyRaw)) / BigInt(10) ** BigInt(baseDecimals);
    if (notional > BigInt(Number.MAX_SAFE_INTEGER)) return null;
    return formatPrice(Number(notional), quoteDecimals);
  }, [price, quantity, quoteDecimals, baseDecimals]);

  useEffect(() => {
    if (priceHint !== null) {
      setPrice(rawToHumanInput(priceHint, quoteDecimals));
    }
  }, [priceHint, priceHintNonce, quoteDecimals]);

  function applyPercent(nextPct: number) {
    setPct(nextPct);
    if (!balances || availableRaw <= 0) {
      setQuantity("");
      return;
    }

    if (side === "sell") {
      const qty = Math.floor((availableRaw * nextPct) / 100);
      const stepped =
        Math.floor(qty / market.quantity_step_size) * market.quantity_step_size;
      setQuantity(stepped > 0 ? rawToHumanInput(stepped, baseDecimals) : "");
      return;
    }

    const priceRaw = parseHumanToRaw(price, quoteDecimals);
    if (priceRaw === null || priceRaw <= 0) {
      setQuantity("");
      return;
    }

    const spend = Math.floor((availableRaw * nextPct) / 100);
    const qty = Number(
      (BigInt(spend) * BigInt(10) ** BigInt(baseDecimals)) / BigInt(priceRaw),
    );
    const stepped =
      Math.floor(qty / market.quantity_step_size) * market.quantity_step_size;
    setQuantity(stepped > 0 ? rawToHumanInput(stepped, baseDecimals) : "");
  }

  async function submit() {
    setError("");
    setOk("");
    const priceRaw = parseHumanToRaw(price, quoteDecimals);
    const qtyRaw = parseHumanToRaw(quantity, baseDecimals);
    if (priceRaw === null || qtyRaw === null) {
      setError("Enter a valid price and quantity.");
      return;
    }
    if (priceRaw % market.price_tick_size !== 0) {
      setError(
        `Price must be a multiple of tick ${formatPrice(market.price_tick_size, quoteDecimals)}.`,
      );
      return;
    }
    if (qtyRaw % market.quantity_step_size !== 0) {
      setError("Quantity must match the market step size.");
      return;
    }
    if (qtyRaw < market.min_order_quantity) {
      setError("Quantity is below the market minimum.");
      return;
    }

    setSubmitting(true);
    try {
      await createOrder({
        market_symbol: market.symbol,
        order_type: side,
        price: priceRaw,
        quantity: qtyRaw,
      });
      setOk("Order accepted — waiting for engine apply.");
      setQuantity("");
      setPct(0);
      onSubmitted?.();
    } catch (caught) {
      setError(
        caught instanceof Error ? caught.message : "Unable to place order.",
      );
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="flex h-full min-h-0 flex-col overflow-hidden rounded-lg border border-[#2c2533] bg-[#100d14]">
      <div className="grid grid-cols-2 gap-1.5 border-b border-[#2c2533] p-2 bg-[#140f1a]">
        <button
          className={`rounded-xl py-2.5 text-sm font-bold transition-all duration-200 ${
            side === "buy"
              ? "bg-[#74ddbd] text-[#0c1b16] shadow-[0_0_16px_rgba(116,221,189,0.35)]"
              : "text-[#8e8594] hover:bg-[#1f1927] hover:text-[#fff8f5]"
          }`}
          onClick={() => {
            setSide("buy");
            setPct(0);
          }}
          type="button"
        >
          Buy {base}
        </button>
        <button
          className={`rounded-xl py-2.5 text-sm font-bold transition-all duration-200 ${
            side === "sell"
              ? "bg-[#ff6f61] text-[#160e12] shadow-[0_0_16px_rgba(255,111,97,0.35)]"
              : "text-[#8e8594] hover:bg-[#1f1927] hover:text-[#fff8f5]"
          }`}
          onClick={() => {
            setSide("sell");
            setPct(0);
          }}
          type="button"
        >
          Sell {base}
        </button>
      </div>

      <div className="flex flex-1 flex-col gap-4 p-4">
        <div className="flex items-center justify-between text-xs">
          <span className="rounded-md border border-[#2b2434] bg-[#18131f] px-2.5 py-1 font-mono text-[10px] font-semibold uppercase tracking-wider text-[#a198a5]">
            Spot / Limit Order
          </span>
          <span className="text-[#8e8594]">
            Avail:{" "}
            <span className="font-mono font-semibold text-[#fff8f5]">
              {availableLabel}
            </span>
          </span>
        </div>

        <AssetField
          label="Limit Price"
          onChange={(value) => {
            setPrice(value);
            setPct(0);
          }}
          placeholder="0.00"
          symbol={quote}
          value={price}
        />

        <div className="space-y-2">
          <AssetField
            label="Quantity"
            onChange={(value) => {
              setQuantity(value);
              setPct(0);
            }}
            placeholder="0.00"
            symbol={base}
            value={quantity}
          />
          <div className="flex items-center gap-1.5 pt-1">
            {PCT_STEPS.map((step) => (
              <button
                className={`h-7 flex-1 rounded-lg text-[11px] font-mono font-semibold transition-all ${
                  pct === step
                    ? side === "buy"
                      ? "bg-[#13211e] text-[#74ddbd] border border-[#1b3d34]"
                      : "bg-[#271a20] text-[#ff8175] border border-[#481f25]"
                    : "bg-[#17131d] text-[#716878] border border-[#261f2e] hover:text-[#dcd4de] hover:border-[#382e42]"
                }`}
                key={step}
                onClick={() => applyPercent(step)}
                type="button"
              >
                {step}%
              </button>
            ))}
          </div>
        </div>

        <AssetField
          label="Est. Total Notional"
          placeholder="—"
          readOnly
          symbol={quote}
          value={orderValueLabel ?? ""}
        />

        {balances === null ? (
          <p className="rounded-xl border border-[#302839] bg-[#17131d] px-3.5 py-3 text-xs leading-5 text-[#817787]">
            <Link
              className="font-semibold text-[#ff8175] hover:underline"
              href="/login"
            >
              Log in
            </Link>{" "}
            to place orders and trade balances.
          </p>
        ) : null}

        {error ? (
          <p className="text-xs font-medium text-[#ff9e96]">{error}</p>
        ) : null}
        {ok ? <p className="text-xs font-medium text-[#74ddbd]">{ok}</p> : null}

        <Button
          className={`mt-auto h-12 w-full rounded-xl text-base font-bold transition-all duration-200 active:scale-[0.98] ${
            side === "buy"
              ? "border-transparent bg-[#74ddbd] text-[#0c1b16] hover:bg-[#86e7c9] shadow-[0_4px_20px_rgba(116,221,189,0.3)]"
              : "border-transparent bg-[#ff6f61] text-[#160e12] hover:bg-[#ff8477] shadow-[0_4px_20px_rgba(255,111,97,0.3)]"
          }`}
          disabled={
            submitting || balances === null || market.status !== "trading"
          }
          onClick={submit}
          tone="quiet"
        >
          {submitting
            ? "Submitting order..."
            : `${side === "buy" ? "Buy" : "Sell"} ${base}`}
        </Button>
      </div>
    </div>
  );
}

function AssetField({
  label,
  symbol,
  value,
  onChange,
  placeholder,
  readOnly = false,
}: {
  label: string;
  symbol: string;
  value: string;
  onChange?: (value: string) => void;
  placeholder: string;
  readOnly?: boolean;
}) {
  return (
    <label className="block">
      <span className="mb-1.5 flex items-center justify-between text-[11px] font-medium text-[#8e8594]">
        <span>{label}</span>
        <span className="font-mono text-[10px] uppercase tracking-[0.08em] text-[#716878]">
          {symbol}
        </span>
      </span>
      <div className="group relative flex h-11 items-center rounded-xl border border-[#302839] bg-[#17131d] transition-all duration-200 focus-within:border-[#ff6f61] focus-within:shadow-[0_0_12px_rgba(255,111,97,0.2)]">
        <input
          className="h-full w-full bg-transparent pr-12 pl-3.5 font-mono text-sm text-[#fff8f5] outline-none placeholder:text-[#4a4352] disabled:cursor-default"
          disabled={readOnly}
          inputMode="decimal"
          onChange={(event) => onChange?.(event.target.value)}
          placeholder={placeholder}
          readOnly={readOnly}
          value={value}
        />
        <span className="pointer-events-none absolute top-1/2 right-3 -translate-y-1/2">
          <AssetIcon size={22} symbol={symbol} />
        </span>
      </div>
    </label>
  );
}
