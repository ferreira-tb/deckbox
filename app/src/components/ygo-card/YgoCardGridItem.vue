<script setup lang="ts">
import type { CardImpl } from '@/lib/model/card';

interface Props {
  card: CardImpl;
  onClick: () => void;
}

defineProps<Props>();
</script>

<template>
  <div>
    <img
      :src="card.imagePathSmall"
      :alt="card.name"
      loading="lazy"
      class="size-full object-cover cursor-pointer"
      @click="onClick"
    />
  </div>
</template>
