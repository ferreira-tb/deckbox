import type { Db_CardId, Wish } from '@/lib/bindings';

export class WishImpl implements Wish {
  public readonly cardId: Db_CardId;

  constructor(wish: Wish) {
    this.cardId = wish.cardId;
  }
}
