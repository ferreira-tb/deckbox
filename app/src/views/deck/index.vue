<script setup lang="ts">
import { storeToRefs } from "pinia";
import type { Option } from "@tb-dev/utils";
import { useSettings } from "@/stores/settings";
import type { CardImpl } from "@/lib/model/card";
import { useDecks } from "@/composables/useDecks";
import { useTrunk } from "@/composables/useTrunk";
import SelectDeck from "@/views/deck/SelectDeck.vue";
import { useCardsInTrunk } from "@/composables/useCardsInTrunk";
import { computed, nextTick, onMounted, ref, shallowRef } from "vue";
import YgoCardGridSide from "@/components/ygo-card/YgoCardGridSide.vue";
import { Button, Input, Table, TableBody, TableCell, TableRow } from "@tb-dev/vue-components";

const settings = useSettings();
const { canEdit } = storeToRefs(settings);

const {
  decks,
  currentDeckId,
  loading,
  createDeck,
  hasDeckName,
} = useDecks();

const cards = useCardsInTrunk();
const selectedCard = shallowRef<Option<CardImpl>>();

const { getTrunkEntryAmount } = useTrunk();

const deckName = ref<Option<string>>();

const disabled = computed(() => !canEdit.value || loading.value);

const isValidDeckName = computed(() => {
  return (
    typeof deckName.value === "string" &&
    deckName.value.length > 0 &&
    !hasDeckName(deckName.value)
  );
});

onMounted(async () => {
  await nextTick();
  selectedCard.value ??= cards.value.at(0);
});

async function create() {
  const name = deckName.value?.trim();
  if (name && !hasDeckName(name)) {
    await createDeck({ name });
    deckName.value = null;
  }
}
</script>

<template>
  <div class="size-full flex gap-2 overflow-hidden">
    <YgoCardGridSide
      v-if="selectedCard"
      :card="selectedCard"
      class="max-w-72 lg:max-w-80"
    />

    <div class="size-full flex flex-col">
      <div class="menu-grid w-full gap-2">
        <Button
          variant="destructive"
          :disabled
        >
          <span>Remove</span>
        </Button>
        <SelectDeck
          :key="decks.length"
          v-model="currentDeckId"
          :decks
          :disabled
        />
        <Button
          variant="default"
          :disabled
        >
          <span>Save</span>
        </Button>

        <Button
          variant="default"
          :disabled="disabled || !isValidDeckName"
        >
          <span>Rename</span>
        </Button>
        <Input
          v-model.trim="deckName"
          type="text"
          :disabled
          :minlength="1"
          :maxlength="30"
        />
        <Button
          variant="default"
          :disabled="disabled || !isValidDeckName"
          @click="create"
        >
          <span>Create</span>
        </Button>
      </div>

      <div class="w-full"></div>
    </div>

    <div class="min-w-max h-full pb-4 overflow-hidden">
      <Table class="size-full max-w-72 lg:max-w-80 overflow-x-hidden overflow-y-auto">
        <TableBody>
          <template v-for="card of cards" :key="card.cardId">
            <TableRow
              class="cursor-pointer"
              @click="() => (selectedCard = card)"
            >
              <TableCell class="max-w-max p-0">
                <span class="px-2">{{ getTrunkEntryAmount(card.cardId) }}</span>
              </TableCell>
              <TableCell>
                <span>{{ card.name }}</span>
              </TableCell>
            </TableRow>
          </template>
        </TableBody>
      </Table>
    </div>
  </div>
</template>

<style scoped>
.menu-grid {
  display: grid;
  grid-template-columns: 100px 1fr 100px;
}
</style>
