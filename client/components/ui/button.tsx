import type { ButtonHTMLAttributes } from "react";

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  tone?: "primary" | "quiet";
};

export function Button({
  className = "",
  tone = "primary",
  ...props
}: ButtonProps) {
  const tones = {
    primary: "bg-[#ff6f61] text-[#160e12] hover:bg-[#ff8a7f] border-[#ff6f61]",
    quiet: "bg-[#17131d] text-[#eee8ef] hover:bg-[#211b29] border-[#302938]",
  };

  return (
    <button
      className={`inline-flex h-11 items-center justify-center gap-2 rounded-full border px-5 text-sm font-semibold transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${tones[tone]} ${className}`}
      {...props}
    />
  );
}
