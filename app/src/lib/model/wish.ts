import type { Wish } from '@/lib/bindings';

export class WishImpl implements Wish {
  public readonly cardId: string;

  constructor(wish: Wish) {
    this.cardId = wish.cardId;
  }
}
