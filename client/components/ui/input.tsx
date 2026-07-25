import type { InputHTMLAttributes } from "react";

type InputProps = InputHTMLAttributes<HTMLInputElement> & {
  label: string;
  hint?: string;
};

export function Input({ label, hint, className = "", ...props }: InputProps) {
  return (
    <label className="block">
      <span className="mb-2 flex items-center justify-between text-xs font-medium uppercase tracking-[0.14em] text-[#aaa0ae]">
        {label}
        {hint && (
          <span className="normal-case tracking-normal text-[#716978]">
            {hint}
          </span>
        )}
      </span>
      <input
        className={`h-12 w-full rounded-xl border border-[#342d3b] bg-[#15111a] px-4 text-sm text-[#fff8f5] outline-none transition-all duration-200 placeholder:text-[#5f5665] focus:border-[#ff6f61] focus:shadow-[0_0_12px_rgba(255,111,97,0.2)] ${className}`}
        {...props}
      />
    </label>
  );
}
