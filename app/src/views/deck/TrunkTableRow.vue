<script setup lang="ts">
import { computed } from "vue";
import type { Option } from "@tb-dev/utils";
import type { CardImpl } from "@/lib/model/card";
import { useDecks } from "@/composables/useDecks";
import { useTrunk } from "@/composables/useTrunk";
import type { Db_CardLocalId } from "@/lib/bindings";
import { cn, TableCell, TableRow } from "@tb-dev/vue-components";

interface Props {
  card: CardImpl;
}

const props = defineProps<Props>();

const selectedCardId = defineModel<Option<Db_CardLocalId>>({ required: true });

const { currentDeck } = useDecks();
const { getTrunkEntryAmount } = useTrunk();

const amount = computed(() => {
  const inDeck = currentDeck.value?.getCard(props.card.id)?.sum() ?? 0;
  const inTrunk = getTrunkEntryAmount(props.card.cardId);
  return Math.max(0, inTrunk - inDeck);
});

const rowClass = computed(() => {
  let value = "hover:bg-background cursor-pointer select-none";
  if (amount.value < 1) {
    value = cn(value, "text-muted-foreground");
  }

  return value;
});
</script>

<template>
  <TableRow
    :data-state="selectedCardId === card.id ? 'selected' : ''"
    :class="rowClass"
    @click="() => void (selectedCardId = card.id)"
  >
    <TableCell class="max-w-min p-0">
      <span class="px-2">{{ amount }}</span>
    </TableCell>
    <TableCell class="w-full">
      <span>{{ card.name }}</span>
    </TableCell>
  </TableRow>
</template>
