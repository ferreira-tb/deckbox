<script setup lang="ts">
import { storeToRefs } from "pinia";
import { onKeyDown } from "@vueuse/core";
import type { Option } from "@tb-dev/utils";
import { onCtrlKeyDown } from "@tb-dev/vue";
import { useSettings } from "@/stores/settings";
import { useDecks } from "@/composables/useDecks";
import type { Db_CardLocalId } from "@/lib/bindings";
import SelectDeck from "@/views/deck/SelectDeck.vue";
import TrunkTable from "@/views/deck/TrunkTable.vue";
import { Button, Input } from "@tb-dev/vue-components";
import { useCardsInTrunk } from "@/composables/useCardsInTrunk";
import type { CardDiff, CardPlacement } from "@/lib/model/deck";
import YgoCardTable from "@/components/ygo-card/YgoCardTable.vue";
import { computed, nextTick, onMounted, ref, shallowRef } from "vue";
import YgoCardGridSide from "@/components/ygo-card/YgoCardGridSide.vue";

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
  clearDeck,
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

const isExtraDeckCard = computed(() => {
  return selectedCard.value?.isExtraDeckCard();
});

const deckNumbers = computed(() => {
  return {
    main: currentDeck.value?.sumMainDeckCards() ?? 0,
    extra: currentDeck.value?.sumExtraDeckCards() ?? 0,
    side: currentDeck.value?.sumSideDeckCards() ?? 0,
  };
});

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

onCtrlKeyDown(["s", "S"], save);
onCtrlKeyDown(["x", "X"], clear);

// This must be imported from '@vueuse/core'.
onKeyDown("1", (e) => void update(e.ctrlKey, "main"));
onKeyDown("2", (e) => void update(e.ctrlKey, "extra"));
onKeyDown("3", (e) => void update(e.ctrlKey, "side"));

async function clear() {
  if (currentDeckId.value) {
    await clearDeck(currentDeckId.value);
  }
}

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

async function update(shouldDecrease: boolean, placement: CardPlacement) {
  if (selectedCardId.value && currentDeckId.value) {
    const diff: CardDiff = { card_id: selectedCardId.value };
    if (shouldDecrease) {
      diff.main = placement === "main" ? -1 : 0;
      diff.extra = placement === "extra" ? -1 : 0;
      diff.side = placement === "side" ? -1 : 0;
    }
    else {
      diff.main = placement === "main" ? 1 : 0;
      diff.extra = placement === "extra" ? 1 : 0;
      diff.side = placement === "side" ? 1 : 0;

      if (deckNumbers.value.main >= 60) {
        diff.main = 0;
      }

      if (deckNumbers.value.extra >= 15) {
        diff.extra = 0;
      }

      if (deckNumbers.value.side >= 15) {
        diff.side = 0;
      }
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
          v-model.trim.stop="deckName"
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

      <div class="size-full grid grid-cols-2 xl:grid-cols-3 gap-2 py-2 lg:py-4 overflow-hidden">
        <div class="flex flex-col gap-2 overflow-hidden">
          <div v-if="currentDeck && deckNumbers.main > 0" class="flex justify-center items-center">
            <span class="text-muted-foreground text-sm">{{ `Main Deck (${deckNumbers.main})` }}</span>
          </div>
          <YgoCardTable
            v-model="selectedCardId"
            :cards="mainDeckCards"
            check-trunk
            :get-amount="(card) => card.main"
            class="overflow-x-hidden overflow-y-auto"
          />
        </div>

        <div class="flex flex-col gap-2 overflow-hidden">
          <div v-if="currentDeck && deckNumbers.extra > 0" class="flex justify-center items-center">
            <span class="text-muted-foreground text-sm">{{ `Extra Deck (${deckNumbers.extra})` }}</span>
          </div>
          <YgoCardTable
            v-model="selectedCardId"
            :cards="extraDeckCards"
            check-trunk
            :get-amount="(card) => card.extra"
            class="overflow-x-hidden overflow-y-auto"
          />
        </div>

        <div class="max-xl:hidden flex flex-col gap-2 overflow-hidden">
          <div v-if="currentDeck && deckNumbers.side > 0" class="flex justify-center items-center">
            <span class="text-muted-foreground text-sm">{{ `Side Deck (${deckNumbers.side})` }}</span>
          </div>
          <YgoCardTable
            v-model="selectedCardId"
            :cards="sideDeckCards"
            check-trunk
            :get-amount="(card) => card.side"
            class="overflow-x-hidden overflow-y-auto"
          />
        </div>
      </div>
    </div>

    <div class="min-w-max flex flex-col gap-2">
      <div class="size-full pb-4 overflow-hidden">
        <TrunkTable v-model="selectedCardId" :cards />
      </div>

      <div class="grid grid-cols-3 gap-2 p-2">
        <Button
          variant="default"
          size="sm"
          :disabled="disabled || !currentDeckId || !selectedCardId || isExtraDeckCard"
          @click="(e: MouseEvent) => update(e.ctrlKey, 'main')"
        >
          <span>Main</span>
        </Button>
        <Button
          variant="default"
          size="sm"
          :disabled="disabled || !currentDeckId || !selectedCardId || !isExtraDeckCard"
          @click="(e: MouseEvent) => update(e.ctrlKey, 'extra')"
        >
          <span>Extra</span>
        </Button>
        <Button
          variant="default"
          size="sm"
          :disabled="disabled || !currentDeckId || !selectedCardId"
          @click="(e: MouseEvent) => update(e.ctrlKey, 'side')"
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
