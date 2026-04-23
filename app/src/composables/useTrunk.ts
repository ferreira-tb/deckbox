import { toast } from '@/lib/toast';
import { handleError } from '@/lib/error';
import type { Option } from '@tb-dev/utils';
import { useDatabase } from '@/composables/useDatabase';
import { tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { TrunkEntryImpl } from '@/lib/model/trunk-entry';
import { commands, type Db_CardId } from '@/lib/bindings';
import { dangerouslyRemoveFromLocalWishlist } from '@/composables/useWishlist';
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

export function useTrunk() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const value = scope.run(create)!;
    return {
      trunk: value.trunk,
      trunkSet: value.trunkSet,
      totalInTrunk: value.totalInTrunk,
      loading: value.loading,
      getTrunkEntry: value.getTrunkEntry,
      getTrunkEntryAmount: value.getTrunkEntryAmount,
      isInTrunk: value.isInTrunk,
      loadTrunk: value.loadTrunk,
      updateTrunkEntryAmount: value.updateTrunkEntryAmount,
    };
  });
}

function create() {
  const trunk = shallowRef<TrunkEntryImpl[]>([]);
  const { locked, ...mutex } = useMutex();

  const trunkSet = computed(() => {
    return new Set(trunk.value.map((entry) => entry.cardId));
  });

  const totalInTrunk = computed(() => {
    return trunk.value.reduce((sum, entry) => sum + entry.amount, 0);
  });

  const { withCard } = useDatabase();

  async function loadTrunk() {
    try {
      await mutex.acquire();
      const result = await commands.getTrunk();
      trunk.value = result.map((entry) => {
        return markRaw(new TrunkEntryImpl(entry));
      });
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  function getTrunkEntry(cardId: Db_CardId) {
    return trunk.value.find((entry) => entry.cardId === cardId) ?? null;
  }

  function getTrunkEntryAmount(cardId: Db_CardId) {
    return getTrunkEntry(cardId)?.amount ?? 0;
  }

  function isInTrunk(cardId: Db_CardId) {
    return trunkSet.value.has(cardId);
  }

  async function updateTrunkEntryAmount(cardId: Db_CardId, kind: 'increase' | 'decrease') {
    try {
      await mutex.acquire();

      let newAmount: Option<number> = null;
      const currentAmount = getTrunkEntryAmount(cardId);

      if (kind === 'increase') {
        if (currentAmount === 0) {
          await createTrunkEntry(cardId);
          dangerouslyRemoveFromLocalWishlist(cardId);
          return;
        }
        else {
          newAmount = await commands.increaseTrunkEntryAmount(cardId);
        }
      }
      else if (currentAmount > 0) {
        newAmount = await commands.decreaseTrunkEntryAmount(cardId);
      }

      if (typeof newAmount === 'number') {
        if (newAmount === 0) {
          trunk.value = trunk.value.filter((entry) => {
            return entry.cardId !== cardId;
          });

          withCard(cardId, (card) => {
            toast.success(`Removed "${card.name}" from trunk`);
          });
        }
        else {
          let entry = getTrunkEntry(cardId);
          if (entry) {
            entry = markRaw(entry.withAmount(newAmount));
            trunk.value = trunk.value
              .filter((it) => it.cardId !== cardId)
              .concat(entry);

            if (newAmount > currentAmount) {
              withCard(cardId, (card) => {
                toast.success(`Added "${card.name}" to trunk`);
              });
            }
            else if (newAmount < currentAmount) {
              withCard(cardId, (card) => {
                toast.success(`Removed "${card.name}" from trunk`);
              });
            }
          }
        }
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  async function createTrunkEntry(cardId: Db_CardId) {
    const rows = await commands.createTrunkEntry(cardId);
    if (rows > 0) {
      const entry = await commands.getTrunkEntryByCardId(cardId);
      trunk.value.push(markRaw(new TrunkEntryImpl(entry)));
      triggerRef(trunk);

      withCard(cardId, (card) => {
        toast.success(`Added "${card.name}" to trunk`);
      });
    }
  }

  return {
    trunk: trunk as Readonly<Ref<readonly TrunkEntryImpl[]>>,
    trunkSet: trunkSet as Readonly<Ref<ReadonlySet<Db_CardId>>>,
    totalInTrunk,
    loading: locked,
    getTrunkEntry,
    getTrunkEntryAmount,
    isInTrunk,
    loadTrunk,
    updateTrunkEntryAmount,
  };
}
