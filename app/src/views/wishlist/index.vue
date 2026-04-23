<script setup lang="ts">
import { sessionRef } from '@tb-dev/vue';
import { Button } from '@tb-dev/vue-components';
import { useTrunk } from '@/composables/useTrunk';
import { useWishlist } from '@/composables/useWishlist';
import { useWishedCards } from '@/composables/useWishedCards';
import YgoCardGrid from '@/components/ygo-card/YgoCardGrid.vue';

const { updateTrunkEntryAmount } = useTrunk();

const {
  loading: isLoadingWishlist,
  isInWishlist,
  removeWish,
} = useWishlist();

const wishedCards = useWishedCards();

const searchValue = sessionRef('search:wishlist', '');
</script>

<template>
  <div class="size-full">
    <YgoCardGrid
      v-model:search="searchValue"
      :cards="wishedCards"
      @remove-wish="removeWish"
      @update-trunk-entry-amount="updateTrunkEntryAmount"
    >
      <template #sideAction="{ cardId, inTrunk }">
        <div class="grid grid-cols-2 justify-center items-center gap-2">
          <Button variant="outline">
            <span v-if="inTrunk === 0">Trunk</span>
            <span v-else>Trunk ({{ inTrunk }})</span>
          </Button>

          <Button
            variant="outline"
            :disabled="isLoadingWishlist || !isInWishlist(cardId)"
            @click="() => removeWish(cardId)"
          >
            <span>Remove</span>
          </Button>
        </div>
      </template>
    </YgoCardGrid>
  </div>
</template>
