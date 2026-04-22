import { handleError } from '@/lib/error';
import { tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { TrunkEntryImpl } from '@/lib/model/trunk-entry';
import { commands, type Db_CardId } from '@/lib/bindings';
import { computed, effectScope, type InjectionKey, markRaw, type Ref, shallowRef } from 'vue';

const SYMBOL = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useTrunk() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const value = scope.run(create)!;
    return {
      trunk: value.trunk,
      trunkSet: value.trunkSet,
      loading: value.loading,
      loadTrunk: value.loadTrunk,
    };
  });
}

function create() {
  const trunk = shallowRef<TrunkEntryImpl[]>([]);
  const { locked, ...mutex } = useMutex();

  const trunkSet = computed(() => {
    return new Set(trunk.value.map((entry) => entry.cardId));
  });

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

  async function addToTrunk(cardId: Db_CardId, amount: number) {
    try {
      await mutex.acquire();
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  return {
    trunk: trunk as Readonly<Ref<readonly TrunkEntryImpl[]>>,
    trunkSet: trunkSet as Readonly<Ref<ReadonlySet<Db_CardId>>>,
    loading: locked,
    loadTrunk,
  };
}
