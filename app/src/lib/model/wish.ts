import type { Db_CardId, Db_Wish } from '@/lib/bindings';

export class WishImpl implements Db_Wish {
  public readonly cardId: Db_CardId;

  constructor(wish: Db_Wish) {
    this.cardId = wish.cardId;
  }
}
