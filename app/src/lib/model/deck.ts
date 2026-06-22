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

  public clear() {
    this.cards = [];
  }

  public getCard(id: Db_CardLocalId) {
    return this.cards.find((card) => card.card_id === id) ?? null;
  }

  public getRawCards() {
    return this.cards.map((card) => card.toJSON());
  }

  public isLegal() {
    return (
      this.cards.every((card) => card.sum() <= 3) &&
      this.sumMainDeckCards() <= 60 &&
      this.sumExtraDeckCards() <= 15 &&
      this.sumSideDeckCards() <= 15
    );
  }

  public normalizeCards() {
    for (const card of this.cards) {
      card.main = clamp(card.main, 0, 3);
      card.extra = clamp(card.extra, 0, 3);
      card.side = clamp(card.side, 0, 3);
    }
  }

  public removeEmptyCards() {
    this.cards = this.cards.filter((card) => !card.isEmpty());
  }

  public sumMainDeckCards() {
    return this.cards.reduce((acc, curr) => acc + curr.main, 0);
  }

  public sumExtraDeckCards() {
    return this.cards.reduce((acc, curr) => acc + curr.extra, 0);
  }

  public sumSideDeckCards() {
    return this.cards.reduce((acc, curr) => acc + curr.side, 0);
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
