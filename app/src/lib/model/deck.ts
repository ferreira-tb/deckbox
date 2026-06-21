import { DeckCardImpl } from "@/lib/model/deck-card";
import { commands, type Db_Deck, type Db_DeckId } from "@/lib/bindings";

export class DeckImpl implements Db_Deck {
  public readonly id: Db_DeckId;
  public readonly name: string;
  public readonly description: string | null;

  public cards: DeckCardImpl[] = [];

  constructor(deck: Db_Deck) {
    this.id = deck.id;
    this.name = deck.name;
    this.description = deck.description;
  }

  public async load() {
    const cards = await commands.getDeckCards(this.id);
    this.cards = cards.map((card) => new DeckCardImpl(card));
  }
}
