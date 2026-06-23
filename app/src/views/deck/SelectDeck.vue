<script setup lang="ts">
import { computed } from "vue";
import type { Option } from "@tb-dev/utils";
import type { Db_Deck, Db_DeckId } from "@/lib/bindings";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@tb-dev/vue-components";

interface Props {
  decks: readonly Db_Deck[];
  disabled: boolean;
}

const props = defineProps<Props>();

const currentDeckId = defineModel<Option<Db_DeckId>>({ required: true });

const id = computed({
  get: () => currentDeckId.value?.toString(10),
  set: (value) => {
    if (value) {
      currentDeckId.value = Number.parseInt(value);
    }
    else {
      currentDeckId.value = null;
    }
  },
});
</script>

<template>
  <Select v-model="id" placeholder="None" :disabled="disabled || decks.length === 0">
    <SelectTrigger class="w-full">
      <SelectValue />
    </SelectTrigger>
    <SelectContent>
      <SelectItem v-for="deck of decks" :key="deck.id" :value="deck.id.toString(10)">
        {{ deck.name }}
      </SelectItem>
    </SelectContent>
  </Select>
</template>
