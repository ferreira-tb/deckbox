import { compare } from "@/lib/intl";
import { handleError } from "@/lib/error";
import { DeckImpl } from "@/lib/model/deck";
import type { Option } from "@tb-dev/utils";
import { mapAsync } from "es-toolkit/array";
import { tryInjectOrElse, useMutex } from "@tb-dev/vue";
import { commands, type Db_DeckId, type Db_NewDeck } from "@/lib/bindings";
import {
  effectScope,
  type InjectionKey,
  markRaw,
  ref,
  type Ref,
  shallowRef,
  triggerRef,
} from "vue";

const SYMBOL = Symbol("decks") as InjectionKey<ReturnType<typeof create>>;

export function useDecks() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const value = scope.run(create)!;
    return {
      decks: value.decks,
      currentDeckId: value.currentDeckId,
      loading: value.loading,
      createDeck: value.createDeck,
      getCurrent: value.getCurrent,
      getDeck: value.getDeck,
      hasDeckName: value.hasDeckName,
      loadDecks: value.loadDecks,
    };
  });
}

function create() {
  const decks = shallowRef<DeckImpl[]>([]);
  const currentId = ref<Option<Db_DeckId>>();

  const { locked, ...mutex } = useMutex();

  async function loadDecks() {
    try {
      await mutex.acquire();
      const result = await commands.getDecks();
      decks.value = await mapAsync(result, async (deck) => {
        const impl = new DeckImpl(deck);
        await impl.load();
        return markRaw(impl);
      });

      sortDecks();
      fallbackDeck();
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  async function createDeck(newDeck: Db_NewDeck) {
    try {
      await mutex.acquire();
      const id = await commands.createDeck(newDeck);
      const deck = await commands.getDeck(id);
      const deckImpl = new DeckImpl(deck);
      await deckImpl.load();

      decks.value.push(markRaw(deckImpl));
      currentId.value = deckImpl.id;

      sortDecks();
      triggerRef(decks);
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  function getCurrent() {
    return currentId.value ? getDeck(currentId.value) : null;
  }

  function getDeck(deckId: Db_DeckId) {
    return decks.value.find((deck) => deck.id === deckId) ?? null;
  }

  function fallbackDeck() {
    if (!currentId.value || decks.value.every((deck) => deck.id !== currentId.value)) {
      currentId.value ??= decks.value.at(0)?.id;
    }
  }

  function hasDeckName(name: string) {
    return decks.value.some((deck) => deck.name === name);
  }

  function sortDecks() {
    decks.value.sort((a, b) => compare(a.name, b.name));
  }

  return {
    decks: decks as Readonly<Ref<readonly DeckImpl[]>>,
    currentDeckId: currentId,
    loading: locked,
    createDeck,
    getCurrent,
    getDeck,
    hasDeckName,
    loadDecks,
  };
}
