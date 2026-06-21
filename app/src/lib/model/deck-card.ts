import type { Db_CardLocalId, Db_DeckCard, Db_DeckCardAmount, Db_DeckId } from "@/lib/bindings";

export class DeckCardImpl implements Db_DeckCard {
  public readonly deck_id: Db_DeckId;
  public readonly card_id: Db_CardLocalId;
  public main: Db_DeckCardAmount;
  public extra: Db_DeckCardAmount;
  public side: Db_DeckCardAmount;

  constructor(card: Db_DeckCard) {
    this.deck_id = card.deck_id;
    this.card_id = card.card_id;
    this.main = card.main;
    this.extra = card.extra;
    this.side = card.side;
  }
}
