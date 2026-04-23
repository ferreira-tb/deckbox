<script setup lang="ts">
import { sessionRef } from '@tb-dev/vue';
import { Button } from '@tb-dev/vue-components';
import { useTrunk } from '@/composables/useTrunk';
import { useDatabase } from '@/composables/useDatabase';
import { useWishlist } from '@/composables/useWishlist';
import { commands, type Db_CardId } from '@/lib/bindings';
import YgoCardGrid from '@/components/ygo-card/YgoCardGrid.vue';

const { cards, totalInDatabase } = useDatabase();

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

const searchValue = sessionRef('search:database', '');

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
      v-model:search="searchValue"
      :cards
      show-trunk
      show-wish
      @add-wish="addWish"
      @buy-card="(id) => commands.openStoreWebsite(id)"
      @remove-wish="removeWish"
      @update-trunk-entry-amount="updateTrunkEntryAmount"
    >
      <template #header>
        <div class="text-sm text-muted-foreground whitespace-nowrap">
          <span v-if="totalInDatabase === 1">1 card</span>
          <span v-else-if="totalInDatabase > 1">{{ `${totalInDatabase} cards` }}</span>
        </div>
      </template>

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
            v-if="inTrunk === 0 && !isInWishlist(cardId)"
            variant="outline"
            :disabled="isLoadingWishlist"
            @click="() => addWish(cardId)"
          >
            <span>Wish</span>
          </Button>
          <Button
            v-else
            variant="outline"
            @click="() => commands.openStoreWebsite(cardId)"
          >
            <span>Buy</span>
          </Button>
        </div>
      </template>
    </YgoCardGrid>
  </div>
</template>
