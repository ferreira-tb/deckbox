import type { Option } from "@tb-dev/utils";
import type { Db_Card } from "@/lib/bindings";
import { computed, type MaybeRefOrGetter, type Ref, toRef } from "vue";

export function useCardCycleList<T extends Db_Card>(
  cards: MaybeRefOrGetter<T[]>,
  current: Ref<Option<T>>,
) {
  const cardsRef = toRef(cards);
  const currentIndex = computed(() => {
    const currentId = current.value?.cardId;
    return cardsRef.value.findIndex((it) => {
      return it.cardId === currentId;
    });
  });

  function go(index: number) {
    const size = cardsRef.value.length;
    index = ((index % size) + size) % size;
    current.value = at(index) ?? at(0);
  }

  function at(index: number) {
    return cardsRef.value.at(index) as Option<T>;
  }

  function next() {
    go(currentIndex.value + 1);
  }

  function previous() {
    go(currentIndex.value - 1);
  }

  function first() {
    go(0);
  }

  function last() {
    go(cardsRef.value.length - 1);
  }

  return {
    currentIndex,
    go,
    next,
    previous,
    first,
    last,
  };
}
