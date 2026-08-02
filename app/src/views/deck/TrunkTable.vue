<script vapor lang="ts">
import { FuseWorker } from "fuse.js/worker";
import { watchDebounced } from "@vueuse/core";
import type { CardImpl } from "@/lib/model/card";
import type { Db_CardLocalId } from "@/lib/bindings";
import type { MaybePromise, Option } from "@tb-dev/utils";
import { Table, TableBody } from "@tb-dev/vue-components";
import TrunkTableRow from "@/views/deck/TrunkTableRow.vue";
import { nextTick, onMounted, onUnmounted, shallowRef, watch } from "vue";

interface Props {
  cards: readonly CardImpl[];
  searchValue: Option<string>;
  onUpdate: (card: CardImpl, shouldDecrease: boolean) => MaybePromise<void>;
}

const props = defineProps<Props>();

const selectedCardId = defineModel<Option<Db_CardLocalId>>({ required: true });

const shownCards = shallowRef<readonly CardImpl[]>([]);

const fuse = new FuseWorker(props.cards, {
  keys: ["name", "namePt", "archetype"],
  threshold: 0.2,
  ignoreLocation: true,
  isCaseSensitive: false,
}, {
  numWorkers: 4,
});

watch(() => props.cards, async (values) => {
  await fuse.setCollection(values);
  await updateShownCards();
});

watchDebounced(() => props.searchValue, updateShownCards, {
  debounce: 100,
  maxWait: 500,
});

onMounted(async () => {
  await fuse.setCollection(props.cards);
  await nextTick(updateShownCards);
});

onUnmounted(() => {
  fuse.terminate();
});

async function updateShownCards() {
  const search = props.searchValue?.trim();
  if (search) {
    const results = await fuse.search(search);
    shownCards.value = results.map((result) => {
      return result.item;
    });
  }
  else {
    shownCards.value = props.cards;
  }
}
</script>

<template>
  <Table class="size-full w-72 lg:w-80 overflow-x-hidden overflow-y-auto">
    <TableBody>
      <template v-for="card of shownCards" :key="card.cardId">
        <TrunkTableRow
          v-model="selectedCardId"
          :card
          :selected-card-id
          @click="() => void (selectedCardId = card.id)"
          @dblclick="(e: MouseEvent) => void onUpdate(card, e.ctrlKey)"
        />
      </template>
    </TableBody>
  </Table>
</template>
