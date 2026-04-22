<script setup lang="ts">
import type { Db_CardId } from '@/lib/bindings';
import { Button } from '@tb-dev/vue-components';
import { useTrunk } from '@/composables/useTrunk';
import YgoCardGrid from '@/components/ygo-card/YgoCardGrid.vue';
import { useCardsInTrunk } from '@/composables/useCardsInTrunk';

const cards = useCardsInTrunk();

const {
  loading: isLoadingTrunk,
  updateTrunkEntryAmount,
} = useTrunk();

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
      @update-trunk-entry-amount="updateTrunkEntryAmount"
    >
      <template #sideAction="{ cardId, inTrunk }">
        <div class="grid grid-cols-2 justify-center items-center gap-2">
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
        </div>
      </template>
    </YgoCardGrid>
  </div>
</template>
