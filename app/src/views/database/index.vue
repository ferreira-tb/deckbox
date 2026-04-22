<script setup lang="ts">
import type { Db_CardId } from '@/lib/bindings';
import { Button } from '@tb-dev/vue-components';
import { useCards } from '@/composables/useCards';
import { useWishlist } from '@/composables/useWishlist';
import YgoCardGrid from '@/components/ygo-card/YgoCardGrid.vue';

const { cards } = useCards();

const { addWish, isInWishlist, loading: isLoadingWishlist } = useWishlist();
</script>

<template>
  <div class="size-full">
    <YgoCardGrid :cards @wish="addWish">
      <template #sideAction="{ cardId, inTrunk }">
        <div class="grid grid-cols-3 justify-center items-center gap-2">
          <Button variant="outline">
            <span v-if="inTrunk === 0">Trunk</span>
            <span v-else>Trunk ({{ inTrunk }})</span>
          </Button>

          <Button variant="outline" :disabled="inTrunk < 1">
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
