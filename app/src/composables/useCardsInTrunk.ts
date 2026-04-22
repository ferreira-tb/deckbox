import { computed, type Ref } from 'vue';
import type { CardImpl } from '@/lib/model/card';
import { useCards } from '@/composables/useCards';
import { useTrunk } from '@/composables/useTrunk';

export function useCardsInTrunk(): Readonly<Ref<readonly CardImpl[]>> {
  const { cards } = useCards();
  const { trunkSet } = useTrunk();

  return computed(() => {
    return cards.value.filter((card) => trunkSet.value.has(card.cardId));
  });
}
