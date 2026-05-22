import { watchImmediate } from "@vueuse/core";
import type { Db_CardId } from "@/lib/bindings";
import { useTrunk } from "@/composables/useTrunk";
import { type MaybeRefOrGetter, readonly, ref, toRef } from "vue";

export function useAmountInTrunk(cardId: MaybeRefOrGetter<Db_CardId>) {
  const cardIdRef = toRef(cardId);
  const { trunk, getTrunkEntryAmount } = useTrunk();

  const amount = ref(0);
  watchImmediate([cardIdRef, trunk], () => {
    amount.value = getTrunkEntryAmount(cardIdRef.value);
  });

  return readonly(amount);
}
