import { computed, type Ref } from "vue";
import type { CardImpl } from "@/lib/model/card";
import { useDatabase } from "@/composables/useDatabase";
import { useWishlist } from "@/composables/useWishlist";

export function useWishedCards(): Readonly<Ref<readonly CardImpl[]>> {
  const { cards } = useDatabase();
  const { wishSet } = useWishlist();

  return computed(() => {
    return cards.value.filter((card) => wishSet.value.has(card.cardId));
  });
}
