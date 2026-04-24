<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { sessionRef } from '@tb-dev/vue';
import { useSettings } from '@/stores/settings';
import { Button } from '@tb-dev/vue-components';
import { useTrunk } from '@/composables/useTrunk';
import { useWishlist } from '@/composables/useWishlist';
import { commands, type Db_CardId } from '@/lib/bindings';
import { useWishedCards } from '@/composables/useWishedCards';
import YgoCardGrid from '@/components/ygo-card/YgoCardGrid.vue';

const settings = useSettings();
const { canEdit } = storeToRefs(settings);

const { updateTrunkEntryAmount, loading: isTrunkLoading } = useTrunk();

const { totalInWishlist, removeWish } = useWishlist();

const wishedCards = useWishedCards();

const searchValue = sessionRef('search:wishlist', '');

async function onUpdateTrunkEntry(cardId: Db_CardId) {
  await updateTrunkEntryAmount(cardId, 'increase');
}
</script>

<template>
  <div class="size-full">
    <YgoCardGrid
      v-model:search="searchValue"
      :cards="wishedCards"
      @buy-card="(id) => commands.openStoreWebsite(id)"
      @remove-wish="removeWish"
      @update-trunk-entry-amount="updateTrunkEntryAmount"
    >
      <template #header>
        <div class="text-sm text-muted-foreground whitespace-nowrap">
          <span v-if="totalInWishlist === 1">1 card</span>
          <span v-else-if="totalInWishlist > 1">{{ `${totalInWishlist} cards` }}</span>
        </div>
      </template>

      <template #sideAction="{ cardId }">
        <div class="grid grid-cols-2 justify-center items-center gap-2">
          <Button
            variant="outline"
            :disabled="!canEdit || isTrunkLoading"
            @click="() => onUpdateTrunkEntry(cardId)"
          >
            <span>Trunk</span>
          </Button>

          <Button variant="outline" @click="() => commands.openStoreWebsite(cardId)">
            <span>Buy</span>
          </Button>
        </div>
      </template>
    </YgoCardGrid>
  </div>
</template>
