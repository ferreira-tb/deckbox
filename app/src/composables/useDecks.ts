import { toast } from "@/lib/toast";
import { compare } from "@/lib/intl";
import { clamp } from "es-toolkit/math";
import { handleError } from "@/lib/error";
import type { Option } from "@tb-dev/utils";
import { mapAsync } from "es-toolkit/array";
import { computedWithControl } from "@vueuse/core";
import { DeckCardImpl } from "@/lib/model/deck-card";
import { useDatabase } from "@/composables/useDatabase";
import { tryInjectOrElse, useMutex } from "@tb-dev/vue";
import { type CardDiff, DeckImpl } from "@/lib/model/deck";
import { commands, type Db_DeckId, type Db_NewDeck } from "@/lib/bindings";
import {
  effectScope,
  type InjectionKey,
  markRaw,
  nextTick,
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
      currentCards: value.currentCards,
      currentDeck: value.currentDeck,
      currentDeckId: value.currentDeckId,
      extraDeckCards: value.extraDeckCards,
      loading: value.loading,
      mainDeckCards: value.mainDeckCards,
      sideDeckCards: value.sideDeckCards,
      clearDeck: value.clearDeck,
      createDeck: value.createDeck,
      getDeck: value.getDeck,
      hasDeckName: value.hasDeckName,
      loadDecks: value.loadDecks,
      removeDeck: value.removeDeck,
      renameDeck: value.renameDeck,
      saveDeck: value.saveDeck,
      updateDeck: value.updateDeck,
    };
  });
}

function create() {
  const decks = shallowRef<DeckImpl[]>([]);
  const currentDeckId = ref<Option<Db_DeckId>>();
  const currentDeck = computedWithControl(currentDeckId, () => {
    return currentDeckId.value ? getDeck(currentDeckId.value) : null;
  });

  const currentCards = computedWithControl<readonly DeckCardImpl[]>(currentDeck, () => {
    const cards = currentDeck.value?.cards ?? [];
    return cards.filter((card) => card.sum() > 0);
  });

  const mainDeckCards = computedWithControl<readonly DeckCardImpl[]>(currentCards, () => {
    return currentCards.value.filter((card) => card.main > 0);
  });

  const extraDeckCards = computedWithControl<readonly DeckCardImpl[]>(currentCards, () => {
    return currentCards.value.filter((card) => card.extra > 0);
  });

  const sideDeckCards = computedWithControl<readonly DeckCardImpl[]>(currentCards, () => {
    return currentCards.value.filter((card) => card.side > 0);
  });

  const { getCardByLocalId } = useDatabase();

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

  async function clearDeck(deckId: Db_DeckId) {
    try {
      await mutex.acquire();
      const deck = getDeck(deckId);

      if (deck) {
        deck.clear();
        triggerRef(decks);

        if (deck.id === currentDeckId.value) {
          await nextTick();
          currentDeck.trigger();
        }
      }
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
      const deck = getDeck(deckId);

      if (deck) {
        await commands.removeDeck(deckId);
        decks.value = decks.value.filter(({ id }) => {
          return id !== deckId;
        });

        if (currentDeckId.value === deckId) {
          currentDeckId.value = null;
        }

        setFallback();

        toast.success(`Deck "${deck.name}" removed`);
      }
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

  async function saveDeck(deckId: Db_DeckId) {
    try {
      await mutex.acquire();
      const deck = getDeck(deckId);

      if (deck) {
        deck.normalizeCards();
        deck.removeEmptyCards();

        if (
          deck.cards.every((card) => card.sum() <= 3) &&
          deck.sumMainDeckCards() <= 60 &&
          deck.sumExtraDeckCards() <= 15 &&
          deck.sumSideDeckCards() <= 15
        ) {
          const cards = deck.getRawCards();
          await commands.setDeckCards(deckId, cards);
          triggerRef(decks);

          toast.success(`Deck "${deck.name}" saved`);
        }
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  async function updateDeck(deckId: Db_DeckId, diff: CardDiff) {
    try {
      diff.main ??= 0;
      diff.extra ??= 0;
      diff.side ??= 0;

      await mutex.acquire();
      const deck = getDeck(deckId);
      const dbCard = getCardByLocalId(diff.card_id);

      if (deck && dbCard) {
        const isExtra = dbCard.isExtraDeckCard();
        const deckCard = deck.getCard(diff.card_id);

        if (deckCard) {
          deckCard.main = isExtra ? 0 : clamp(deckCard.main + diff.main, 0, 3);
          deckCard.extra = isExtra ? clamp(deckCard.extra + diff.extra, 0, 3) : 0;
          deckCard.side = clamp(deckCard.side + diff.side, 0, 3);
        }
        else {
          const newDeckCard = new DeckCardImpl({
            deck_id: deck.id,
            card_id: diff.card_id,
            main: isExtra ? 0 : clamp(diff.main, 0, 3),
            extra: isExtra ? clamp(diff.extra, 0, 3) : 0,
            side: clamp(diff.side, 0, 3),
          });

          if (!newDeckCard.isEmpty() && newDeckCard.sum() <= 3) {
            deck.cards.push(newDeckCard);
          }
        }

        deck.normalizeCards();

        if (currentDeck.value?.id === deck.id) {
          currentDeck.trigger();
          await nextTick();

          if (diff.main !== 0) {
            mainDeckCards.trigger();
          }

          if (diff.extra !== 0) {
            extraDeckCards.trigger();
          }

          if (diff.side !== 0) {
            sideDeckCards.trigger();
          }
        }
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
    currentCards,
    currentDeck,
    currentDeckId,
    extraDeckCards,
    loading: locked,
    mainDeckCards,
    sideDeckCards,
    clearDeck,
    createDeck,
    getDeck,
    hasDeckName,
    loadDecks,
    removeDeck,
    renameDeck,
    saveDeck,
    updateDeck,
  };
}
