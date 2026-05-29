import { toast } from "@/lib/toast";
import { storeToRefs } from "pinia";
import { handleError } from "@/lib/error";
import { WishImpl } from "@/lib/model/wish";
import { useSettings } from "@/stores/settings";
import { useDatabase } from "@/composables/useDatabase";
import { tryInjectOrElse, useMutex } from "@tb-dev/vue";
import { commands, type Db_CardId } from "@/lib/bindings";
import {
  computed,
  effectScope,
  type InjectionKey,
  markRaw,
  nextTick,
  type Ref,
  shallowRef,
  triggerRef,
} from "vue";

const SYMBOL = Symbol("wishlist") as InjectionKey<ReturnType<typeof create>>;

export function useWishlist() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const value = scope.run(create)!;
    return {
      wishlist: value.wishlist,
      wishSet: value.wishSet,
      loading: value.loading,
      totalInWishlist: value.totalInWishlist,
      addWish: value.addWish,
      dangerouslyRemoveLocalWish: value.dangerouslyRemoveLocalWish,
      getWish: value.getWish,
      isInWishlist: value.isInWishlist,
      loadWishlist: value.loadWishlist,
      removeWish: value.removeWish,
    };
  });
}

function create() {
  const wishlist = shallowRef<WishImpl[]>([]);
  const { locked, ...mutex } = useMutex();

  const wishSet = computed(() => {
    return new Set(wishlist.value.map((wish) => wish.cardId));
  });

  const totalInWishlist = computed(() => {
    return wishlist.value.length;
  });

  const settings = useSettings();
  const { canEdit } = storeToRefs(settings);

  const { withCard } = useDatabase();

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

  function getWish(cardId: Db_CardId) {
    return wishlist.value.find((wish) => wish.cardId === cardId) ?? null;
  }

  function isInWishlist(cardId: Db_CardId) {
    return wishSet.value.has(cardId);
  }

  async function addWish(cardId: Db_CardId) {
    if (canEdit.value && !isInWishlist(cardId)) {
      try {
        await mutex.acquire();
        const wishId = await commands.createWish(cardId);
        if (wishId) {
          const wish = await commands.getWishByCardId(cardId);
          wishlist.value.push(markRaw(new WishImpl(wish)));
          triggerRef(wishlist);

          withCard(cardId, (card) => {
            toast.success(`Added "${card.name}" to wishlist`);
          });
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
    if (canEdit.value && isInWishlist(cardId)) {
      try {
        await mutex.acquire();
        const rows = await commands.removeWish(cardId);
        if (rows > 0) {
          dangerouslyRemoveLocalWish(cardId);
          withCard(cardId, (card) => {
            toast.success(`Removed "${card.name}" from wishlist`);
          });
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

  function dangerouslyRemoveLocalWish(cardId: Db_CardId) {
    if (canEdit.value) {
      wishlist.value = wishlist.value.filter((wish) => {
        return wish.cardId !== cardId;
      });
    }
  }

  return {
    wishlist: wishlist as Readonly<Ref<readonly WishImpl[]>>,
    wishSet: wishSet as Readonly<Ref<ReadonlySet<Db_CardId>>>,
    loading: locked,
    totalInWishlist,
    addWish,
    dangerouslyRemoveLocalWish,
    getWish,
    isInWishlist,
    loadWishlist,
    removeWish,
  };
}

export function dangerouslyRemoveFromLocalWishlist(cardId: Db_CardId) {
  void nextTick(() => {
    useWishlist().dangerouslyRemoveLocalWish(cardId);
  });
}
