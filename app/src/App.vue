<script setup lang="ts">
import { go } from '@/router';
import { onMounted } from 'vue';
import { commands } from '@/lib/bindings';
import { handleError } from '@/lib/error';
import { RefreshCwIcon } from '@lucide/vue';
import { useColorMode } from '@vueuse/core';
import Loading from '@/components/Loading.vue';
import { throttle } from 'es-toolkit/function';
import { useTrunk } from '@/composables/useTrunk';
import { exit } from '@tauri-apps/plugin-process';
import { useDatabase } from '@/composables/useDatabase';
import { useWishlist } from '@/composables/useWishlist';
import { onKeyDown, useBreakpoints, useMutex } from '@tb-dev/vue';
import NavigationMenuItem from '@/components/NavigationMenuItem.vue';
import { Button, NavigationMenu, NavigationMenuList, Sonner } from '@tb-dev/vue-components';

const { md } = useBreakpoints();

const { locked, lock } = useMutex();

const { loadCards } = useDatabase();
const { loadTrunk } = useTrunk();
const { loadWishlist } = useWishlist();

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('F1', () => go('trunk'));
onKeyDown('F2', () => go('deck'));
onKeyDown('F3', () => go('wishlist'));
onKeyDown('F4', () => go('database'));
onKeyDown('F5', throttle(loadData, 1000));
onKeyDown('F6', throttle(refresh, 5000));
onKeyDown('Escape', () => exit(0).err());

onMounted(async () => {
  try {
    await loadData();
  }
  catch (err) {
    handleError(err);
  }
  finally {
    void commands.showWindow();
  }
});

async function refresh() {
  await lock(async () => {
    try {
      await commands.fetchCards();
      await loadData();
    }
    catch (err) {
      handleError(err);
    }
  });
}

async function loadData() {
  await loadCards();
  await loadTrunk();
  await loadWishlist();
}
</script>

<template>
  <main class="fixed inset-0 select-none pb-safe">
    <Sonner :position="md ? 'bottom-right' : 'top-center'" />
    <div class="size-full flex flex-col overflow-hidden">
      <div class="flex justify-between items-center p-2">
        <div class="w-full">
          <NavigationMenu>
            <NavigationMenuList>
              <NavigationMenuItem route="trunk" label="Trunk" />
              <NavigationMenuItem route="deck" label="Decks" />
              <NavigationMenuItem route="wishlist" label="Wishlist" />
              <NavigationMenuItem route="database" label="Database" />
            </NavigationMenuList>
          </NavigationMenu>
        </div>

        <div class="flex items-center gap-2 pr-2">
          <Button variant="outline" :disabled="locked" @click="refresh">
            <RefreshCwIcon class="size-6" />
          </Button>
        </div>
      </div>

      <div class="relative size-full overflow-hidden">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <KeepAlive>
              <Suspense>
                <component :is="Component" />
                <template #fallback>
                  <Loading class="absolute inset-0" />
                </template>
              </Suspense>
            </KeepAlive>
          </template>
        </RouterView>
      </div>
    </div>
  </main>
</template>
