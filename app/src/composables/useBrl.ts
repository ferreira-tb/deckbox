import { isToday } from "date-fns";
import { asyncRef } from "@tb-dev/vue";
import type { Option } from "@tb-dev/utils";
import { fetch } from "@tauri-apps/plugin-http";
import { computed, type MaybeRefOrGetter, toRef } from "vue";

const STORAGE_KEY = "exchange-rate";

export function useBrl(price: MaybeRefOrGetter<Option<string>>) {
  const priceRef = toRef(price);
  const { state: rate } = asyncRef(null, getExchangeRate);

  return computed(() => {
    if (priceRef.value && rate.value) {
      const priceVal = Number.parseFloat(priceRef.value);
      const bid = Number.parseFloat(rate.value.bid);
      if (Number.isFinite(priceVal) && Number.isFinite(bid)) {
        const result = priceVal * bid;
        return result > 0 ? result : null;
      }
    }

    return null;
  });
}

interface ExchangeRate {
  readonly timestamp: number;
  readonly value: ExchangeRateValue;
}

/** @see https://docs.awesomeapi.com.br/api-de-moedas#formato-de-resposta */
interface ExchangeRateValue {
  readonly high: string;
  readonly low: string;
  readonly bid: string;
  readonly ask: string;
}

async function getExchangeRate() {
  const rateStr = localStorage.getItem(STORAGE_KEY);
  if (rateStr) {
    const rate: ExchangeRate = JSON.parse(rateStr);
    if (isToday(new Date(rate.timestamp))) {
      return rate.value;
    }
  }

  const response = await fetch("https://economia.awesomeapi.com.br/last/USD-BRL");
  if (response.ok) {
    const { USDBRL }: { USDBRL: ExchangeRateValue; } = await response.json();
    const rate: ExchangeRate = { timestamp: Date.now(), value: USDBRL };
    localStorage.setItem(STORAGE_KEY, JSON.stringify(rate, null, 0));
    return USDBRL;
  }
  else {
    return null;
  }
}
