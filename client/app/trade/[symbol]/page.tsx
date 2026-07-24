import { TradeWorkspace } from "@/components/trade/trade-workspace";

type Props = {
  params: Promise<{ symbol: string }>;
};

export default async function TradePage({ params }: Props) {
  const { symbol } = await params;
  return <TradeWorkspace symbol={symbol} />;
}
