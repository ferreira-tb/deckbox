<script setup lang="ts">
import type { Option } from "@tb-dev/utils";
import type { CardImpl } from "@/lib/model/card";
import type { Db_CardLocalId } from "@/lib/bindings";
import { Table, TableBody } from "@tb-dev/vue-components";
import TrunkTableRow from "@/views/deck/TrunkTableRow.vue";

interface Props {
  cards: readonly CardImpl[];
}

const props = defineProps<Props>();

const selectedCardId = defineModel<Option<Db_CardLocalId>>({ required: true });
</script>

<template>
  <Table class="size-full max-w-72 lg:max-w-80 overflow-x-hidden overflow-y-auto">
    <TableBody>
      <template v-for="card of cards" :key="card.cardId">
        <TrunkTableRow v-model="selectedCardId" :card />
      </template>
    </TableBody>
  </Table>
</template>
