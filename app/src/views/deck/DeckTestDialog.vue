<script setup lang="ts">
import { onKeyDown } from "@tb-dev/vue";
import { shuffle } from "es-toolkit/array";
import type { Option } from "@tb-dev/utils";
import type { CardImpl } from "@/lib/model/card";
import type { DeckImpl } from "@/lib/model/deck";
import type { Db_CardLocalId } from "@/lib/bindings";
import { useDatabase } from "@/composables/useDatabase";
import YgoCardGridSide from "@/components/ygo-card/YgoCardGridSide.vue";
import { computed, type CSSProperties, nextTick, shallowRef, type VNode, watch } from "vue";
import { Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogTrigger } from "@tb-dev/vue-components";

interface Props {
  deck: Option<DeckImpl>;
}

const props = defineProps<Props>();

const isOpen = defineModel<boolean>("open", { required: true });

defineSlots<{
  trigger?: () => VNode;
}>();

const { getCardByLocalId } = useDatabase();

const hand = shallowRef<readonly CardImpl[]>([]);

const cards = computed(() => {
  const ids: Db_CardLocalId[] = [];
  for (const card of (props.deck?.cards ?? [])) {
    if (card.main > 0) {
      for (let i = 0; i < card.main; i++) {
        ids.push(card.card_id);
      }
    }
  }

  return ids;
});

const selectedCardId = shallowRef<Option<Db_CardLocalId>>();
const selectedCard = computed(() => getCardByLocalId(selectedCardId.value));

const gridStyle = computed<CSSProperties>(() => {
  const handSize = hand.value.length;
  return {
    gridTemplateColumns: `repeat(${handSize}, minmax(0, 1fr))`,
  };
});

watch(isOpen, async (value) => {
  hand.value = [];
  selectedCardId.value = null;
  if (value) {
    await nextTick(test);
  }
});

onKeyDown(["t", "T"], test, { enabled: isOpen });
onKeyDown("1", () => selectAt(0), { enabled: isOpen });
onKeyDown("2", () => selectAt(1), { enabled: isOpen });
onKeyDown("3", () => selectAt(2), { enabled: isOpen });
onKeyDown("4", () => selectAt(3), { enabled: isOpen });
onKeyDown("5", () => selectAt(4), { enabled: isOpen });

function test() {
  const newHand: CardImpl[] = [];
  for (const card of shuffle(cards.value)) {
    const impl = getCardByLocalId(card);
    if (impl?.isMainDeckCard()) {
      newHand.push(impl);
    }

    if (newHand.length >= 5) {
      break;
    }
  }

  hand.value = newHand;
  selectedCardId.value = newHand.at(0)?.id;
}

function selectAt(idx: number) {
  const impl = hand.value.at(idx);
  if (impl) {
    selectedCardId.value = impl.id;
  }
}
</script>

<template>
  <Dialog v-model:open="isOpen">
    <DialogTrigger v-if="$slots.trigger" as-child>
      <slot name="trigger"></slot>
    </DialogTrigger>

    <DialogContent class="min-w-[80vw]! max-w-[80vw]!">
      <div class="flex gap-2 p-4">
        <YgoCardGridSide
          v-if="selectedCard"
          :card="selectedCard"
          hide-image
          hide-price
          class="max-w-72 lg:max-w-80 h-[50vh]!"
        />
        <div class="size-full grid items-center gap-2 select-none" :style="gridStyle">
          <img
            v-for="(card, idx) of hand"
            :key="`${card.id}+${idx}`"
            :src="card.imagePathSmall"
            :alt="card.name"
            loading="eager"
            class="cursor-pointer object-scale-down"
            @click="() => void (selectedCardId = card.id)"
          />
        </div>
      </div>

      <DialogFooter class="select-none">
        <DialogClose as-child>
          <Button variant="outline">
            <span>Cancel</span>
          </Button>
        </DialogClose>
        <Button variant="default" @click="test">
          <span>Test hand</span>
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
