import { clamp } from "es-toolkit/math";
import type { Option } from "@tb-dev/utils";
import { DeckCardImpl } from "@/lib/model/deck-card";
import {
  commands,
  type Db_CardLocalId,
  type Db_Deck,
  type Db_DeckCardAmount,
  type Db_DeckId,
} from "@/lib/bindings";

export class DeckImpl implements Db_Deck {
  public readonly id: Db_DeckId;
  public name: string;
  public description: string | null;
  public cards: DeckCardImpl[] = [];

  constructor(deck: Db_Deck) {
    this.id = deck.id;
    this.name = deck.name;
    this.description = deck.description;
  }

  public getCard(id: Db_CardLocalId) {
    return this.cards.find((card) => card.card_id === id) ?? null;
  }

  public removeEmptyCards() {
    this.cards = this.cards.filter((card) => !card.isEmpty());
  }

  public update(diff: CardDiff) {
    const card = this.getCard(diff.card_id);
    diff.main ??= 0;
    diff.extra ??= 0;
    diff.side ??= 0;

    if (card) {
      card.main = clamp(card.main + diff.main, 0, 3);
      card.extra = clamp(card.extra + diff.extra, 0, 3);
      card.side = clamp(card.side + diff.side, 0, 3);
    }
    else {
      const newCard = new DeckCardImpl({
        deck_id: this.id,
        card_id: diff.card_id,
        main: clamp(diff.main, 0, 3),
        extra: clamp(diff.extra, 0, 3),
        side: clamp(diff.side, 0, 3),
      });

      if (!newCard.isEmpty()) {
        this.cards.push(newCard);
      }
    }
  }

  public async load() {
    const cards = await commands.getDeckCards(this.id);
    this.cards = cards.map((card) => new DeckCardImpl(card));
  }
}

export interface CardDiff {
  readonly card_id: Db_CardLocalId;
  main?: Option<Db_DeckCardAmount>;
  extra?: Option<Db_DeckCardAmount>;
  side?: Option<Db_DeckCardAmount>;
}

export type CardPlacement = Exclude<keyof CardDiff, "card_id">;
