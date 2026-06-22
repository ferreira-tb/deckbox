<script setup lang="ts">
import { storeToRefs } from "pinia";
import type { Option } from "@tb-dev/utils";
import { useSettings } from "@/stores/settings";
import { useDecks } from "@/composables/useDecks";
import { useTrunk } from "@/composables/useTrunk";
import type { Db_CardLocalId } from "@/lib/bindings";
import SelectDeck from "@/views/deck/SelectDeck.vue";
import { useCardsInTrunk } from "@/composables/useCardsInTrunk";
import type { CardDiff, CardPlacement } from "@/lib/model/deck";
import YgoCardTable from "@/components/ygo-card/YgoCardTable.vue";
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
  sideDeckCards,
  createDeck,
  hasDeckName,
  removeDeck,
  renameDeck,
  saveDeck,
  updateDeck,
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
  setCardFallback();
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

async function save() {
  if (currentDeckId.value) {
    await saveDeck(currentDeckId.value);
  }
}

async function update(e: MouseEvent, placement: CardPlacement) {
  if (selectedCardId.value && currentDeckId.value) {
    const diff: CardDiff = { card_id: selectedCardId.value };
    if (e.ctrlKey) {
      diff.main = placement === "main" ? -1 : 0;
      diff.extra = placement === "extra" ? -1 : 0;
      diff.side = placement === "side" ? -1 : 0;
    }
    else {
      diff.main = placement === "main" ? 1 : 0;
      diff.extra = placement === "extra" ? 1 : 0;
      diff.side = placement === "side" ? 1 : 0;
    }

    await updateDeck(currentDeckId.value, diff);
  }
}

function setCardFallback() {
  selectedCardId.value ??= mainDeckCards.value.at(0)?.card_id ??
    extraDeckCards.value.at(0)?.card_id ??
    sideDeckCards.value.at(0)?.card_id ??
    cards.value.at(0)?.id;
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
          <YgoCardTable
            v-model="selectedCardId"
            :cards="mainDeckCards"
            :get-amount="(card) => card.main"
          />
        </div>

        <div>
          <YgoCardTable
            v-model="selectedCardId"
            :cards="extraDeckCards"
            :get-amount="(card) => card.extra"
          />
        </div>
      </div>
    </div>

    <div class="min-w-max flex flex-col gap-2">
      <div class="size-full pb-4 overflow-hidden">
        <Table class="size-full max-w-72 lg:max-w-80 overflow-x-hidden overflow-y-auto">
          <TableBody>
            <template v-for="card of cards" :key="card.cardId">
              <TableRow
                :data-state="selectedCardId === card.id ? 'selected' : ''"
                class="hover:bg-background cursor-pointer select-none"
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

      <div class="grid grid-cols-3 gap-2 p-2">
        <Button
          variant="default"
          size="sm"
          :disabled="disabled || !currentDeckId || !selectedCardId"
          @click="(e: MouseEvent) => update(e, 'main')"
        >
          <span>Main</span>
        </Button>
        <Button
          variant="default"
          size="sm"
          :disabled="disabled || !currentDeckId || !selectedCardId"
          @click="(e: MouseEvent) => update(e, 'extra')"
        >
          <span>Extra</span>
        </Button>
        <Button
          variant="default"
          size="sm"
          :disabled="disabled || !currentDeckId || !selectedCardId"
          @click="(e: MouseEvent) => update(e, 'side')"
        >
          <span>Side</span>
        </Button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.menu-grid {
  display: grid;
  grid-template-columns: 100px 1fr 100px;
}
</style>
