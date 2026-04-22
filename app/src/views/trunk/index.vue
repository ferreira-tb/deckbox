<script setup lang="ts">
import { Button } from '@tb-dev/vue-components';
import YgoCardGrid from '@/components/ygo-card/YgoCardGrid.vue';
import { useCardsInTrunk } from '@/composables/useCardsInTrunk';

const cards = useCardsInTrunk();
</script>

<template>
  <div class="size-full">
    <YgoCardGrid :cards>
      <template #sideAction="{ inTrunk }">
        <div class="grid grid-cols-2 justify-center items-center gap-2">
          <Button variant="outline">
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
