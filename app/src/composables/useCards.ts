import { commands } from '@/lib/bindings';
import { handleError } from '@/lib/error';
import { CardImpl } from '@/lib/model/card';
import { tryOnMounted } from '@vueuse/core';
import { tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { effectScope, type InjectionKey, markRaw, type Ref, shallowRef } from 'vue';

const SYMBOL = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useCards() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const value = scope.run(create)!;
    return {
      cards: value.cards,
      loading: value.loading,
      loadCards: value.loadCards,
    };
  });
}

function create() {
  const cards = shallowRef<CardImpl[]>([]);
  const { locked, ...mutex } = useMutex();

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

  tryOnMounted(() => {
    if (cards.value.length === 0) {
      void loadCards();
    }
  });

  return {
    cards: cards as Readonly<Ref<readonly CardImpl[]>>,
    loading: locked,
    loadCards,
  };
}
