import { handleError } from '@/lib/error';
import { WishImpl } from '@/lib/model/wish';
import { tryOnMounted } from '@vueuse/core';
import { tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { commands, type Db_CardId } from '@/lib/bindings';
import {
  computed,
  effectScope,
  type InjectionKey,
  markRaw,
  type Ref,
  shallowRef,
  triggerRef,
} from 'vue';

const SYMBOL = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useWishlist() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const value = scope.run(create)!;
    return {
      wishlist: value.wishlist,
      wishSet: value.wishSet,
      loading: value.loading,
      loadWishlist: value.loadWishlist,
      addWish: value.addWish,
      removeWish: value.removeWish,
      isInWishlist: value.isInWishlist,
    };
  });
}

function create() {
  const wishlist = shallowRef<WishImpl[]>([]);
  const { locked, ...mutex } = useMutex();

  const wishSet = computed(() => {
    return new Set(wishlist.value.map((wish) => wish.cardId));
  });

  async function loadWishlist() {
    try {
      await mutex.acquire();
      const result = await commands.getWishlist();
      wishlist.value = result.map((wish) => {
        return markRaw(new WishImpl(wish));
      });
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  async function addWish(cardId: Db_CardId) {
    if (!isInWishlist(cardId)) {
      try {
        await mutex.acquire();
        const rows = await commands.createWish(cardId);
        if (rows > 0) {
          const wish = await commands.getWishByCardId(cardId);
          wishlist.value.push(markRaw(new WishImpl(wish)));
          triggerRef(wishlist);
        }
      }
      catch (err) {
        handleError(err);
      }
      finally {
        mutex.release();
      }
    }
  }

  async function removeWish(cardId: Db_CardId) {
    if (isInWishlist(cardId)) {
      try {
        await mutex.acquire();
        const rows = await commands.removeWish(cardId);
        if (rows > 0) {
          wishlist.value = wishlist.value.filter((wish) => wish.cardId !== cardId);
        }
      }
      catch (err) {
        handleError(err);
      }
      finally {
        mutex.release();
      }
    }
  }

  function isInWishlist(cardId: Db_CardId) {
    return wishSet.value.has(cardId);
  }

  tryOnMounted(() => {
    if (wishlist.value.length === 0) {
      void loadWishlist();
    }
  });

  return {
    wishlist: wishlist as Readonly<Ref<readonly WishImpl[]>>,
    wishSet: wishSet as Readonly<Ref<ReadonlySet<Db_CardId>>>,
    loading: locked,
    loadWishlist,
    addWish,
    removeWish,
    isInWishlist,
  };
}
