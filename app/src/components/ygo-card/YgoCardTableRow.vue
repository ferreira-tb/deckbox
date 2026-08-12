<script setup lang="ts">
import { computed } from "vue";
import type { Option } from "@tb-dev/utils";
import { useSettings } from "@/stores/settings";
import { useTrunk } from "@/composables/useTrunk";
import type { Db_CardLocalId } from "@/lib/bindings";
import { useDatabase } from "@/composables/useDatabase";
import type { DeckCardImpl } from "@/lib/model/deck-card";
import { cn, TableCell, TableRow } from "@tb-dev/vue-components";

interface Props {
  card: DeckCardImpl;
  getAmount: (card: DeckCardImpl) => number;
}

const props = defineProps<Props>();

const cardLocalId = defineModel<Option<Db_CardLocalId>>({ required: true });

const settings = useSettings();
const { getCardByLocalId } = useDatabase();
const { getTrunkEntryAmount } = useTrunk();

const dbCard = computed(() => getCardByLocalId(props.card.card_id));
const hasEnoughInTrunk = computed(() => {
  if (settings.checkTrunk) {
    const inTrunk = dbCard.value ? getTrunkEntryAmount(dbCard.value.cardId) : 0;
    return inTrunk >= props.card.sum();
  }
  else {
    return true;
  }
});
</script>

<template>
  <TableRow
    :class="cn('cursor-pointer', !hasEnoughInTrunk && 'text-red-400')"
    @click="() => (cardLocalId = card.card_id)"
  >
    <TableCell class="max-w-max p-0">
      <span class="px-2">{{ getAmount(card) }}</span>
    </TableCell>
    <TableCell>
      <span>{{ dbCard?.name ?? "" }}</span>
    </TableCell>
  </TableRow>
</template>
