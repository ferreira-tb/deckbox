<script setup lang="ts">
import { computed, type VNode } from 'vue';
import type { Option } from '@tb-dev/utils';
import { Badge } from '@tb-dev/vue-components';
import type { Db_CardId } from '@/lib/bindings';
import type { CardImpl } from '@/lib/model/card';
import { useAmountInTrunk } from '@/composables/useAmountInTrunk';

interface Props {
  card: CardImpl;
}

interface ActionSlotProps {
  cardId: Db_CardId;
  inTrunk: number;
  price: Option<number>;
}

const props = defineProps<Props>();

defineSlots<{
  action?: (props: ActionSlotProps) => VNode;
}>();

const cardId = computed(() => props.card.cardId);
const inTrunk = useAmountInTrunk(cardId);

const price = computed(() => {
  if (props.card.price) {
    const value = Number.parseFloat(props.card.price);
    if (Number.isFinite(value) && value > 0) {
      return value;
    }
  }

  return null;
});

const description = computed(() => {
  return props.card.description
    .split('\n')
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
});
</script>

<template>
  <div class="h-full flex flex-col gap-2 p-2">
    <img :src="card.imagePath" :alt="card.name" loading="eager" class="max-h-2/4 object-scale-down" />
    <div class="h-full flex flex-col justify-between gap-1 overflow-hidden">
      <div class="flex flex-col gap-1 overflow-x-hidden overflow-y-auto pr-2">
        <div class="flex justify-between gap-2">
          <h1 class="font-bold">{{ card.name }}</h1>
          <Badge v-if="price" variant="outline" class="max-h-max">
            <span>{{ `$${price.toFixed(2)}` }}</span>
          </Badge>
        </div>

        <p v-for="(line, idx) of description" :key="idx">{{ line }}</p>
      </div>

      <slot
        v-if="$slots.action"
        name="action"
        v-bind="{ cardId, inTrunk, price }"
      ></slot>
    </div>
  </div>
</template>
