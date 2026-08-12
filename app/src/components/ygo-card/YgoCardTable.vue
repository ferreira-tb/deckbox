<script setup lang="ts">
import type { Option } from "@tb-dev/utils";
import type { Db_CardLocalId } from "@/lib/bindings";
import type { DeckCardImpl } from "@/lib/model/deck-card";
import { Table, TableBody } from "@tb-dev/vue-components";
import YgoCardTableRow from "@/components/ygo-card/YgoCardTableRow.vue";

interface Props {
  cards: readonly DeckCardImpl[];
  getAmount: (card: DeckCardImpl) => number;
}

const props = defineProps<Props>();

const cardLocalId = defineModel<Option<Db_CardLocalId>>({ required: true });
</script>

<template>
  <Table class="size-full">
    <TableBody>
      <template v-for="card of cards" :key="card.card_id">
        <YgoCardTableRow
          v-if="card.sum() > 0"
          v-model="cardLocalId"
          :card
          :get-amount
        />
      </template>
    </TableBody>
  </Table>
</template>
