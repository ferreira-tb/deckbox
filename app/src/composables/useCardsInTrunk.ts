import { computed, type Ref } from 'vue';
import type { CardImpl } from '@/lib/model/card';
import { useTrunk } from '@/composables/useTrunk';
import { useDatabase } from '@/composables/useDatabase';

export function useCardsInTrunk(): Readonly<Ref<readonly CardImpl[]>> {
  const { cards } = useDatabase();
  const { trunkSet } = useTrunk();

  return computed(() => {
    return cards.value.filter((card) => trunkSet.value.has(card.cardId));
  });
}
