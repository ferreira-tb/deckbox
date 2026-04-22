import type { Db_CardId } from '@/lib/bindings';
import { useTrunk } from '@/composables/useTrunk';
import { computed, type MaybeRefOrGetter, toRef } from 'vue';

export function useAmountInTrunk(cardId: MaybeRefOrGetter<Db_CardId>) {
  const { trunk } = useTrunk();
  const cardIdRef = toRef(cardId);

  return computed(() => {
    const entry = trunk.value.find((item) => item.cardId === cardIdRef.value);
    return entry?.amount ?? 0;
  });
}
