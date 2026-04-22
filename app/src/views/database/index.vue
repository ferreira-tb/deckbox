<script setup lang="ts">
import type { Db_CardId } from '@/lib/bindings';
import { Button } from '@tb-dev/vue-components';
import { useTrunk } from '@/composables/useTrunk';
import { useDatabase } from '@/composables/useDatabase';
import { useWishlist } from '@/composables/useWishlist';
import YgoCardGrid from '@/components/ygo-card/YgoCardGrid.vue';

const { cards } = useDatabase();

const {
  loading: isLoadingTrunk,
  updateTrunkEntryAmount,
} = useTrunk();

const {
  loading: isLoadingWishlist,
  addWish,
  isInWishlist,
  removeWish,
} = useWishlist();

async function onUpdateTrunkEntry(e: MouseEvent, cardId: Db_CardId) {
  if (e.ctrlKey) {
    await updateTrunkEntryAmount(cardId, 'decrease');
  }
  else {
    await updateTrunkEntryAmount(cardId, 'increase');
  }
}
</script>

<template>
  <div class="size-full">
    <YgoCardGrid
      :cards
      show-trunk
      show-wish
      @add-wish="addWish"
      @remove-wish="removeWish"
      @update-trunk-entry-amount="updateTrunkEntryAmount"
    >
      <template #sideAction="{ cardId, inTrunk }">
        <div class="grid grid-cols-3 justify-center items-center gap-2">
          <Button
            variant="outline"
            :disabled="isLoadingTrunk"
            @click="(e: MouseEvent) => onUpdateTrunkEntry(e, cardId)"
          >
            <span v-if="inTrunk === 0">Trunk</span>
            <span v-else>Trunk ({{ inTrunk }})</span>
          </Button>

          <Button variant="outline" disabled>
            <span>Deck</span>
          </Button>

          <Button
            variant="outline"
            :disabled="isLoadingWishlist || inTrunk !== 0 || isInWishlist(cardId)"
            @click="() => addWish(cardId)"
          >
            <span>Wish</span>
          </Button>
        </div>
      </template>
    </YgoCardGrid>
  </div>
</template>
