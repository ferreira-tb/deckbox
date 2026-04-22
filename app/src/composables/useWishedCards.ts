import { computed, type Ref } from 'vue';
import type { CardImpl } from '@/lib/model/card';
import { useCards } from '@/composables/useCards';
import { useWishlist } from '@/composables/useWishlist';

export function useWishedCards(): Readonly<Ref<readonly CardImpl[]>> {
  const { cards } = useCards();
  const { wishSet } = useWishlist();

  return computed(() => {
    return cards.value.filter((card) => wishSet.value.has(card.cardId));
  });
}
