<script setup lang="ts">
import { computed, type VNode } from "vue";
import type { Option } from "@tb-dev/utils";
import { useBrl } from "@/composables/useBrl";
import { Badge } from "@tb-dev/vue-components";
import type { Db_CardId } from "@/lib/bindings";
import type { CardImpl } from "@/lib/model/card";
import { useAmountInTrunk } from "@/composables/useAmountInTrunk";

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

const price = useBrl(() => props.card.price);

const description = computed(() => {
  return props.card.description
    .split("\n")
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
          <h1 class="font-bold select-text">{{ card.name }}</h1>
          <div class="flex justify-end items-center gap-1">
            <Badge
              v-if="card.banlistStatus === 'Forbidden'"
              variant="outline"
              class="max-h-max text-red-500"
            >
              <span>F</span>
            </Badge>
            <Badge
              v-else-if="card.banlistStatus === 'Limited'"
              variant="outline"
              class="max-h-max text-orange-500"
            >
              <span>L</span>
            </Badge>
            <Badge
              v-else-if="card.banlistStatus === 'Semi-Limited'"
              variant="outline"
              class="max-h-max text-yellow-500"
            >
              <span>S</span>
            </Badge>

            <Badge v-if="price" variant="outline" class="max-h-max">
              <span>{{ `R$${price.toFixed(2)}` }}</span>
            </Badge>
          </div>
        </div>

        <p v-for="(line, idx) of description" :key="idx" class="select-text">{{ line }}</p>
      </div>

      <slot
        v-if="$slots.action"
        name="action"
        v-bind="{ cardId, inTrunk, price }"
      ></slot>
    </div>
  </div>
</template>
