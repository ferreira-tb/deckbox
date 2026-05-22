import type {
  Db_CardId,
  Db_TrunkEntry,
  Db_TrunkEntryAmount,
  Db_TrunkEntryId,
} from "@/lib/bindings";

export class TrunkEntryImpl implements Db_TrunkEntry {
  public readonly id: Db_TrunkEntryId;
  public readonly cardId: Db_CardId;
  public readonly amount: Db_TrunkEntryAmount;

  constructor(trunkEntry: Db_TrunkEntry) {
    this.id = trunkEntry.id;
    this.cardId = trunkEntry.cardId;
    this.amount = trunkEntry.amount;
  }

  public withAmount(amount: Db_TrunkEntryAmount): TrunkEntryImpl {
    amount = Math.trunc(Math.max(0, amount));
    return new TrunkEntryImpl({
      id: this.id,
      cardId: this.cardId,
      amount,
    });
  }
}
