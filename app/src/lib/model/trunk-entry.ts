import type { Db_CardId, Db_TrunkEntryAmount, TrunkEntry } from '@/lib/bindings';

export class TrunkEntryImpl implements TrunkEntry {
  public readonly cardId: Db_CardId;
  public readonly amount: Db_TrunkEntryAmount;

  constructor(trunkEntry: TrunkEntry) {
    this.cardId = trunkEntry.cardId;
    this.amount = trunkEntry.amount;
  }

  public withAmount(amount: Db_TrunkEntryAmount): TrunkEntryImpl {
    amount = Math.trunc(Math.max(0, amount));
    return new TrunkEntryImpl({ cardId: this.cardId, amount });
  }
}
