<script setup lang="ts">
import { chunk } from 'es-toolkit/array';
import type { Option } from '@tb-dev/utils';
import { watchDebounced } from '@vueuse/core';
import type { Db_CardId } from '@/lib/bindings';
import type { CardImpl } from '@/lib/model/card';
import { onCtrlKeyDown, onKeyDown } from '@tb-dev/vue';
import { useWishlist } from '@/composables/useWishlist';
import { useCardCycleList } from '@/composables/useCardCycleList';
import YgoCardGridItem from '@/components/ygo-card/YgoCardGridItem.vue';
import YgoCardGridSide from '@/components/ygo-card/YgoCardGridSide.vue';
import YgoCardGridSearch from '@/components/ygo-card/YgoCardGridSearch.vue';
import YgoCardGridPagination from '@/components/ygo-card/YgoCardGridPagination.vue';
import { computed, onBeforeMount, ref, shallowRef, useTemplateRef, type VNode, watch } from 'vue';

interface Props {
  cards: readonly CardImpl[];
  itemsPerPage?: number;
  showTrunk?: boolean;
  showWish?: boolean;

  onAddWish?: (cardId: Db_CardId) => void;
  onRemoveWish?: (cardId: Db_CardId) => void;
  onUpdateTrunkEntryAmount?: (cardId: Db_CardId, kind: 'increase' | 'decrease') => void;
}

interface SideActionSlotProps {
  cardId: Db_CardId;
  inTrunk: number;
}

const props = withDefaults(defineProps<Props>(), {
  itemsPerPage: 300,
});

const searchValue = defineModel<string>('search', { required: true });

defineSlots<{
  sideAction?: (props: SideActionSlotProps) => VNode;
}>();

const shownCards = shallowRef<readonly CardImpl[]>([]);
const selected = shallowRef<Option<CardImpl>>();

const chunks = computed(() => {
  const map = new Map<number, CardImpl[]>();
  const arrays = chunk(shownCards.value, props.itemsPerPage);
  arrays.forEach((array, index) => map.set(index + 1, array));
  return map;
});

const currentPage = ref(1);
const currentChunk = computed(() => {
  return chunks.value.get(currentPage.value) ?? [];
});

const cycleList = useCardCycleList(currentChunk, selected);

const { isInWishlist } = useWishlist();

const searchInput = useTemplateRef('searchInputEl');

const grid = useTemplateRef('gridEl');

watch(() => props.cards, updateShownCards);

watchDebounced(searchValue, updateShownCards, {
  debounce: 300,
  maxWait: 1000,
});

watch(currentPage, () => {
  // eslint-disable-next-line @typescript-eslint/no-unsafe-call
  grid.value?.scrollTo({ top: 0, behavior: 'instant' });
  fallbackSelect();
});

onKeyDown('ArrowLeft', cycleList.previous);
onKeyDown('ArrowRight', cycleList.next);
onKeyDown('ArrowUp', cycleList.first);
onKeyDown('ArrowDown', cycleList.last);

onCtrlKeyDown(['f', 'F'], () => {
  // eslint-disable-next-line
  searchInput.value?.focus();
});

onKeyDown(['t', 'T'], () => {
  if (selected.value) {
    const cardId = selected.value.cardId;
    props.onUpdateTrunkEntryAmount?.(cardId, 'increase');
  }
});

onCtrlKeyDown(['t', 'T'], () => {
  if (selected.value) {
    const cardId = selected.value.cardId;
    props.onUpdateTrunkEntryAmount?.(cardId, 'decrease');
  }
});

onKeyDown(['w', 'W'], () => {
  if (selected.value) {
    if (isInWishlist(selected.value.cardId)) {
      props.onRemoveWish?.(selected.value.cardId);
    }
    else {
      props.onAddWish?.(selected.value.cardId);
    }
  }
});

onBeforeMount(updateShownCards);

function updateShownCards() {
  if (searchValue.value) {
    const value = searchValue.value.toLowerCase();
    shownCards.value = props.cards.filter((card) => {
      return card.name.toLowerCase().includes(value);
    });
  }
  else {
    shownCards.value = props.cards;
  }

  fallbackSelect();
}

function fallbackSelect() {
  if (selected.value) {
    if (!isCardBeingShown(selected.value)) {
      const card = shownCards.value.at(0);
      if (card) selected.value = card;
    }
  }
  else {
    selected.value = currentChunk.value.at(0) ?? shownCards.value.at(0);
  }
}

function isCardBeingShown(card: CardImpl) {
  return shownCards.value.some((it) => it.cardId === card.cardId);
}
</script>

<template>
  <div class="size-full flex flex-col">
    <div class="flex items-center py-2 px-4">
      <YgoCardGridSearch ref="searchInputEl" v-model="searchValue" class="w-68" />
    </div>

    <div class="size-full flex overflow-hidden">
      <YgoCardGridSide v-if="selected" class="max-w-72 lg:max-w-80" :card="selected">
        <template #action="{ cardId, inTrunk }">
          <slot
            v-if="$slots.sideAction"
            name="sideAction"
            v-bind="{ cardId, inTrunk }"
          ></slot>
        </template>
      </YgoCardGridSide>

      <div class="size-full flex flex-col justify-between overflow-hidden">
        <div
          v-if="currentChunk.length > 0"
          ref="gridEl"
          class="grid grid-cols-6 sm:grid-cols-8 xl:grid-cols-10 gap-2 p-1 md:px-4 overflow-x-hidden overflow-y-auto"
        >
          <div v-for="(card, idx) of currentChunk" :key="idx">
            <YgoCardGridItem
              :card
              :show-trunk
              :show-wish
              @click="() => void (selected = card)"
            />
          </div>
        </div>

        <div
          v-if="currentChunk.length > 0 && chunks.size > 1"
          class="flex justify-center py-3"
        >
          <YgoCardGridPagination
            v-model:page="currentPage"
            :items-per-page="props.itemsPerPage"
            :total="shownCards.length"
          />
        </div>
      </div>
    </div>
  </div>
</template>
