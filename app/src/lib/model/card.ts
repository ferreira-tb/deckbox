import { convertFileSrc } from "@tauri-apps/api/core";
import type {
  CardType,
  Db_BanlistStatus,
  Db_Card,
  Db_CardAttribute,
  Db_CardId,
  Db_CardLocalId,
  Db_CardRace,
  Db_CardType,
} from "@/lib/bindings";

export class CardImpl implements Db_Card {
  public readonly id: Db_CardLocalId;
  public readonly name: string;
  public readonly description: string;
  public readonly cardId: Db_CardId;
  public readonly cardType: Db_CardType;
  public readonly cardTypeHuman: string | null;
  public readonly cardRace: Db_CardRace;
  public readonly attack: number | null;
  public readonly defense: number | null;
  public readonly level: number | null;
  public readonly linkval: number | null;
  public readonly attribute: Db_CardAttribute | null;
  public readonly archetype: string | null;
  public readonly banlistStatus: Db_BanlistStatus | null;
  public readonly imageUrl: string;
  public readonly imageUrlCropped: string;
  public readonly imageUrlSmall: string;
  public readonly price: string | null;
  public readonly namePt: string | null;
  public readonly descriptionPt: string | null;
  public readonly ygoprodeckUrl: string | null;

  public readonly imagePath: string;
  public readonly imagePathCropped: string;
  public readonly imagePathSmall: string;

  constructor(card: Db_Card) {
    this.id = card.id;
    this.name = card.name;
    this.description = card.description;
    this.cardId = card.cardId;
    this.cardType = card.cardType;
    this.cardTypeHuman = card.cardTypeHuman;
    this.cardRace = card.cardRace;
    this.attack = card.attack;
    this.defense = card.defense;
    this.level = card.level;
    this.linkval = card.linkval;
    this.attribute = card.attribute;
    this.archetype = card.archetype;
    this.banlistStatus = card.banlistStatus;
    this.imageUrl = card.imageUrl;
    this.imageUrlCropped = card.imageUrlCropped;
    this.imageUrlSmall = card.imageUrlSmall;
    this.price = card.price;
    this.namePt = card.namePt;
    this.descriptionPt = card.descriptionPt;
    this.ygoprodeckUrl = card.ygoprodeckUrl;

    this.imagePath = path(__DECKBOX_IMG_DIR__, this.cardId);
    this.imagePathCropped = path(__DECKBOX_IMG_DIR_CROPPED__, this.cardId);
    this.imagePathSmall = path(__DECKBOX_IMG_DIR_SMALL__, this.cardId);
  }

  public isMainDeckCard() {
    return !this.isExtraDeckCard();
  }

  public isExtraDeckCard() {
    return CardImpl.EXTRA_DECK_TYPES.has(this.cardType);
  }

  public static readonly EXTRA_DECK_TYPES: ReadonlySet<CardType> = new Set(
    [
      "Fusion Monster",
      "Link Monster",
      "Pendulum Effect Fusion Monster",
      "Synchro Monster",
      "Synchro Pendulum Effect Monster",
      "Synchro Tuner Monster",
      "XYZ Monster",
      "XYZ Pendulum Effect Monster",
    ] satisfies readonly CardType[],
  );
}

function path(dir: string, cardId: string) {
  return convertFileSrc(`${dir}/${cardId}.jpg`);
}
