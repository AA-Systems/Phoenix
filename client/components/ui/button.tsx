import type { ButtonHTMLAttributes } from "react";

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  tone?: "primary" | "quiet" | "buy" | "sell";
};

export function Button({
  className = "",
  tone = "primary",
  ...props
}: ButtonProps) {
  const tones = {
    primary:
      "bg-[#ff6f61] text-[#160e12] hover:bg-[#ff8477] border-[#ff6f61] shadow-[0_4px_16px_rgba(255,111,97,0.22)] hover:shadow-[0_6px_24px_rgba(255,111,97,0.35)]",
    quiet:
      "bg-[#17131d]/90 text-[#eee8ef] hover:bg-[#231c2d] border-[#302938] hover:border-[#42384c]",
    buy: "bg-[#74ddbd] text-[#0c1b16] hover:bg-[#86e7c9] border-[#74ddbd] shadow-[0_4px_16px_rgba(116,221,189,0.22)] hover:shadow-[0_6px_24px_rgba(116,221,189,0.35)]",
    sell: "bg-[#ff6f61] text-[#160e12] hover:bg-[#ff8477] border-[#ff6f61] shadow-[0_4px_16px_rgba(255,111,97,0.22)] hover:shadow-[0_6px_24px_rgba(255,111,97,0.35)]",
  };

  return (
    <button
      className={`inline-flex h-11 items-center justify-center gap-2 rounded-full border px-5 text-sm font-semibold transition-all duration-200 active:scale-[0.98] disabled:cursor-not-allowed disabled:opacity-50 ${tones[tone]} ${className}`}
      {...props}
    />
  );
}
