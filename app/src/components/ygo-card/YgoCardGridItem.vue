<script setup lang="ts">
import { cn } from "@tb-dev/vue-components";
import type { CardImpl } from "@/lib/model/card";
import { useWishlist } from "@/composables/useWishlist";
import { useAmountInTrunk } from "@/composables/useAmountInTrunk";

interface Props {
  card: CardImpl;
  showTrunk?: boolean;
  showWish?: boolean;

  onClick: () => void;
}

const props = defineProps<Props>();

const inTrunk = useAmountInTrunk(() => props.card.cardId);

const { isInWishlist } = useWishlist();
</script>

<template>
  <div :class="cn('relative', showWish && isInWishlist(card.cardId) ? 'ring-3 ring-yellow-500' : '')">
    <img
      :src="card.imagePathSmall"
      :alt="card.name"
      loading="lazy"
      class="size-full object-cover cursor-pointer"
      @click="onClick"
    />
    <div v-if="showTrunk && inTrunk > 0">
      <div class="absolute top-1 left-1 size-5 text-sm font-extrabold bg-gray-900/90 text-white rounded">
        <div class="size-full flex items-center justify-center">
          <span>{{ inTrunk }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
