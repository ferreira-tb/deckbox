<script setup lang="ts">
import type { Option } from "@tb-dev/utils";
import type { Db_CardLocalId } from "@/lib/bindings";
import { useDatabase } from "@/composables/useDatabase";
import type { DeckCardImpl } from "@/lib/model/deck-card";
import { Table, TableBody, TableCell, TableRow } from "@tb-dev/vue-components";

interface Props {
  cards: readonly DeckCardImpl[];
  getAmount: (card: DeckCardImpl) => number;
}

const props = defineProps<Props>();

const cardId = defineModel<Option<Db_CardLocalId>>({ required: true });

const { getCardByLocalId } = useDatabase();
</script>

<template>
  <Table class="size-full">
    <TableBody>
      <template v-for="card of cards" :key="card.card_id">
        <TableRow
          class="cursor-pointer"
          @click="() => (cardId = card.card_id)"
        >
          <TableCell class="max-w-max p-0">
            <span class="px-2">{{ getAmount(card) }}</span>
          </TableCell>
          <TableCell>
            <span>{{ getCardByLocalId(card.card_id)?.name ?? "" }}</span>
          </TableCell>
        </TableRow>
      </template>
    </TableBody>
  </Table>
</template>
