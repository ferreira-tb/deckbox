import { compare } from "@/lib/intl";
import { handleError } from "@/lib/error";
import { DeckImpl } from "@/lib/model/deck";
import type { Option } from "@tb-dev/utils";
import { mapAsync } from "es-toolkit/array";
import { tryInjectOrElse, useMutex } from "@tb-dev/vue";
import { commands, type Db_DeckId, type Db_NewDeck } from "@/lib/bindings";
import {
  computed,
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
      currentDeck: value.currentDeck,
      currentDeckId: value.currentDeckId,
      loading: value.loading,
      createDeck: value.createDeck,
      getDeck: value.getDeck,
      hasDeckName: value.hasDeckName,
      loadDecks: value.loadDecks,
      removeDeck: value.removeDeck,
      renameDeck: value.renameDeck,
    };
  });
}

function create() {
  const decks = shallowRef<DeckImpl[]>([]);
  const currentDeckId = ref<Option<Db_DeckId>>();
  const currentDeck = computed(() => {
    return currentDeckId.value ? getDeck(currentDeckId.value) : null;
  });

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
      setFallback();
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
      currentDeckId.value = deckImpl.id;

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

  async function removeDeck(deckId: Db_DeckId) {
    try {
      await mutex.acquire();
      await commands.removeDeck(deckId);
      decks.value = decks.value.filter((deck) => {
        return deck.id !== deckId;
      });

      if (currentDeckId.value === deckId) {
        currentDeckId.value = null;
      }

      setFallback();
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  async function renameDeck(deckId: Db_DeckId, name: string) {
    try {
      await mutex.acquire();
      await commands.renameDeck(deckId, name);

      const deck = getDeck(deckId);
      if (deck) {
        deck.name = name;
        sortDecks();
        triggerRef(decks);
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  function getDeck(deckId: Db_DeckId) {
    return decks.value.find((deck) => deck.id === deckId) ?? null;
  }

  function hasDeckName(name: string) {
    return decks.value.some((deck) => deck.name === name);
  }

  function setFallback() {
    if (!currentDeckId.value || decks.value.every((deck) => deck.id !== currentDeckId.value)) {
      currentDeckId.value ??= decks.value.at(0)?.id;
    }
  }

  function sortDecks() {
    decks.value.sort((a, b) => compare(a.name, b.name));
  }

  return {
    decks: decks as Readonly<Ref<readonly DeckImpl[]>>,
    currentDeck,
    currentDeckId,
    loading: locked,
    createDeck,
    getDeck,
    hasDeckName,
    loadDecks,
    removeDeck,
    renameDeck,
  };
}
