import type { Db_CardId, Db_Wish, Db_WishId } from '@/lib/bindings';

export class WishImpl implements Db_Wish {
  public readonly id: Db_WishId;
  public readonly cardId: Db_CardId;

  constructor(wish: Db_Wish) {
    this.id = wish.id;
    this.cardId = wish.cardId;
  }
}
