import { handleError } from "@/lib/error";
import { CardImpl } from "@/lib/model/card";
import { tryInjectOrElse, useMutex } from "@tb-dev/vue";
import { commands, type Db_CardId } from "@/lib/bindings";
import { computed, effectScope, type InjectionKey, markRaw, type Ref, shallowRef } from "vue";

const SYMBOL = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useDatabase() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const value = scope.run(create)!;
    return {
      cards: value.cards,
      loading: value.loading,
      totalInDatabase: value.totalInDatabase,
      getCard: value.getCard,
      loadCards: value.loadCards,
      withCard: value.withCard,
    };
  });
}

function create() {
  const cards = shallowRef<CardImpl[]>([]);
  const { locked, ...mutex } = useMutex();

  const totalInDatabase = computed(() => {
    return cards.value.length;
  });

  async function loadCards() {
    try {
      await mutex.acquire();
      const result = await commands.getCards();
      cards.value = result.map((card) => {
        return markRaw(new CardImpl(card));
      });
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  function getCard(cardId: Db_CardId) {
    return cards.value.find((card) => card.cardId === cardId) ?? null;
  }

  function withCard<T>(cardId: Db_CardId, fn: (card: CardImpl) => T): T | null {
    const card = getCard(cardId);
    return card ? fn(card) : null;
  }

  return {
    cards: cards as Readonly<Ref<readonly CardImpl[]>>,
    loading: locked,
    totalInDatabase,
    getCard,
    loadCards,
    withCard,
  };
}
