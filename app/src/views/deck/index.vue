<script setup lang="ts">
import { storeToRefs } from "pinia";
import type { Option } from "@tb-dev/utils";
import { useSettings } from "@/stores/settings";
import { useDecks } from "@/composables/useDecks";
import { useTrunk } from "@/composables/useTrunk";
import CardTable from "@/views/deck/CardTable.vue";
import type { Db_CardLocalId } from "@/lib/bindings";
import SelectDeck from "@/views/deck/SelectDeck.vue";
import { useCardsInTrunk } from "@/composables/useCardsInTrunk";
import { computed, nextTick, onMounted, ref, shallowRef } from "vue";
import YgoCardGridSide from "@/components/ygo-card/YgoCardGridSide.vue";
import { Button, Input, Table, TableBody, TableCell, TableRow } from "@tb-dev/vue-components";

const settings = useSettings();
const { canEdit } = storeToRefs(settings);

const {
  decks,
  currentDeck,
  currentDeckId,
  extraDeckCards,
  loading,
  mainDeckCards,
  createDeck,
  hasDeckName,
  removeDeck,
  renameDeck,
} = useDecks();

const cards = useCardsInTrunk();
const selectedCardId = shallowRef<Option<Db_CardLocalId>>();
const selectedCard = computed(() => {
  return selectedCardId.value ?
    cards.value.find((card) => card.id === selectedCardId.value) :
    null;
});

const { getTrunkEntryAmount } = useTrunk();

const deckName = ref<Option<string>>();
const isValidDeckName = computed(() => {
  return (
    typeof deckName.value === "string" &&
    deckName.value.length > 0 &&
    !hasDeckName(deckName.value)
  );
});

const disabled = computed(() => !canEdit.value || loading.value);

const selectDeckKey = computed(() => {
  return `${decks.value.length}+${currentDeck.value?.name ?? ""}`;
});

onMounted(async () => {
  await nextTick();
  selectedCardId.value ??= cards.value.at(0)?.id;
});

async function create() {
  const name = deckName.value?.trim();
  if (name && !hasDeckName(name)) {
    await createDeck({ name });
    deckName.value = null;
  }
}

async function remove() {
  if (currentDeckId.value) {
    await removeDeck(currentDeckId.value);
  }
}

async function rename() {
  const name = deckName.value?.trim();
  if (currentDeckId.value && name && !hasDeckName(name)) {
    await renameDeck(currentDeckId.value, name);
  }
}

async function save() {}
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
          :disabled="disabled || !currentDeckId"
          @click="remove"
        >
          <span>Remove</span>
        </Button>
        <SelectDeck
          :key="selectDeckKey"
          v-model="currentDeckId"
          :decks
          :disabled
        />
        <Button
          variant="default"
          :disabled="disabled || !currentDeckId"
          @click="save"
        >
          <span>Save</span>
        </Button>

        <Button
          variant="default"
          :disabled="disabled || !isValidDeckName"
          @click="rename"
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

      <div class="size-full flex gap-2">
        <div>
          <CardTable
            v-model="selectedCardId"
            :cards="mainDeckCards"
            :get-amount="(card) => card.main"
          />
        </div>

        <div>
          <CardTable
            v-model="selectedCardId"
            :cards="extraDeckCards"
            :get-amount="(card) => card.extra"
          />
        </div>
      </div>
    </div>

    <div class="min-w-max h-full pb-4 overflow-hidden">
      <Table class="size-full max-w-72 lg:max-w-80 overflow-x-hidden overflow-y-auto">
        <TableBody>
          <template v-for="card of cards" :key="card.cardId">
            <TableRow
              class="cursor-pointer"
              @click="() => (selectedCardId = card.id)"
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
