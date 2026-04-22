import type { TrunkEntry } from '@/lib/bindings';

export class TrunkEntryImpl implements TrunkEntry {
  public readonly cardId: string;
  public readonly amount: number;

  constructor(trunkEntry: TrunkEntry) {
    this.cardId = trunkEntry.cardId;
    this.amount = trunkEntry.amount;
  }
}
