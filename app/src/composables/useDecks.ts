import { toast } from "@/lib/toast";
import { compare } from "@/lib/intl";
import { clamp } from "es-toolkit/math";
import { handleError } from "@/lib/error";
import type { Option } from "@tb-dev/utils";
import { watchImmediate } from "@vueuse/core";
import { useTrunk } from "@/composables/useTrunk";
import { DeckCardImpl } from "@/lib/model/deck-card";
import { useDatabase } from "@/composables/useDatabase";
import { tryInjectOrElse, useMutex } from "@tb-dev/vue";
import { type CardDiff, DeckImpl } from "@/lib/model/deck";
import { commands, type Db_DeckId, type Db_NewDeck } from "@/lib/bindings";
import { computed, effectScope, type InjectionKey, ref, type Ref } from "vue";

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
  const decks = ref<DeckImpl[]>([]);
  const currentDeckId = ref<Option<Db_DeckId>>();
  const currentDeck = computed(() => {
    return currentDeckId.value ? getDeck(currentDeckId.value) : null;
  });

  const currentCards = computed<readonly DeckCardImpl[]>(() => {
    const cards = currentDeck.value?.cards ?? [];
    return cards.filter((card) => card.sum() > 0);
  });

  const mainDeckCards = computed<readonly DeckCardImpl[]>(() => {
    return currentCards.value.filter((card) => card.main > 0);
  });

  const extraDeckCards = computed<readonly DeckCardImpl[]>(() => {
    return currentCards.value.filter((card) => card.extra > 0);
  });

  const sideDeckCards = computed<readonly DeckCardImpl[]>(() => {
    return currentCards.value.filter((card) => card.side > 0);
  });

  const { getCardByLocalId } = useDatabase();
  const { getTrunkEntryAmount } = useTrunk();

  const { locked, ...mutex } = useMutex();

  watchImmediate(currentDeck, async (deck) => {
    await deck?.load();
  });

  async function loadDecks() {
    try {
      await mutex.acquire();
      const result = await commands.getDecks();
      decks.value = result.map((deck) => {
        return new DeckImpl(deck);
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

      decks.value.push(deckImpl);
      currentDeckId.value = deckImpl.id;

      sortDecks();
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
      getDeck(deckId)?.clear();
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

        if (deck.isLegal()) {
          const cards = deck.getRawCards();
          await commands.setDeckCards(deckId, cards);

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
        const deckCard = deck.getCard(diff.card_id);
        const isExtra = dbCard.isExtraDeckCard();
        const inTrunk = getTrunkEntryAmount(dbCard.cardId);

        if (deckCard) {
          const main = isExtra ? 0 : clamp(deckCard.main + diff.main, 0, 3);
          const extra = isExtra ? clamp(deckCard.extra + diff.extra, 0, 3) : 0;
          const side = clamp(deckCard.side + diff.side, 0, 3);
          const total = main + extra + side;

          if (total <= 3 && (total <= inTrunk || total < deckCard.sum())) {
            deckCard.main = main;
            deckCard.extra = extra;
            deckCard.side = side;
          }
        }
        else {
          const newDeckCard = new DeckCardImpl({
            deck_id: deck.id,
            card_id: diff.card_id,
            main: isExtra ? 0 : clamp(diff.main, 0, 3),
            extra: isExtra ? clamp(diff.extra, 0, 3) : 0,
            side: clamp(diff.side, 0, 3),
          });

          const total = newDeckCard.sum();
          if (!newDeckCard.isEmpty() && total <= 3 && total <= inTrunk) {
            deck.cards.push(newDeckCard);
          }
        }

        deck.normalizeCards();
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
