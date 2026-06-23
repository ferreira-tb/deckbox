<script setup lang="ts">
import { useRoute } from "vue-router";
import { commands } from "@/lib/bindings";
import { handleError } from "@/lib/error";
import { go, type Route } from "@/router";
import { computed, onMounted } from "vue";
import { useColorMode } from "@vueuse/core";
import Loading from "@/components/Loading.vue";
import { throttle } from "es-toolkit/function";
import { useSettings } from "@/stores/settings";
import { useDecks } from "@/composables/useDecks";
import { useTrunk } from "@/composables/useTrunk";
import { exit } from "@tauri-apps/plugin-process";
import { useDatabase } from "@/composables/useDatabase";
import { useWishlist } from "@/composables/useWishlist";
import NavigationMenuItem from "@/components/NavigationMenuItem.vue";
import { onCtrlKeyDown, onKeyDown, useBreakpoints, useMutex } from "@tb-dev/vue";
import { Button, NavigationMenu, NavigationMenuList, Sonner } from "@tb-dev/vue-components";
import { DatabaseBackupIcon, FileInputIcon, RefreshCwIcon, SettingsIcon } from "@lucide/vue";

const settings = useSettings();

const { md } = useBreakpoints();

const { locked, lock } = useMutex();

const { loadCards } = useDatabase();
const { loadDecks } = useDecks();
const { loadTrunk } = useTrunk();
const { loadWishlist } = useWishlist();

const isDev = globalThis.__DEBUG_ASSERTIONS__;
const version = globalThis.__VERSION__;

const route = useRoute();
const canExportJson = computed(() => {
  const routeName = route.name as Route;
  return (
    routeName === "deck" ||
    routeName === "trunk"
  );
});

useColorMode({
  initialValue: "dark",
  onError: handleError,
  writeDefaults: true,
});

onKeyDown("F1", () => go("trunk"));
onKeyDown("F2", () => go("deck"));
onKeyDown("F3", () => go("wishlist"));
onKeyDown("F4", () => go("database"));
onKeyDown("F5", throttle(loadData, 1000));
onKeyDown("F6", throttle(refresh, 5000));
onKeyDown("Escape", () => exit(0).err());

onCtrlKeyDown(",", () => commands.openSettingsFile().err());
onCtrlKeyDown(["e", "E"], () => void settings.toggleEdit());

onMounted(() => {
  loadData()
    .catch((err: unknown) => handleError(err))
    .finally(() => void commands.showWindow());
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
  await Promise.all([
    loadCards(),
    loadDecks(),
    loadTrunk(),
    loadWishlist(),
  ]);
}

async function exportJson() {
  // eslint-disable-next-line @typescript-eslint/switch-exhaustiveness-check
  switch (route.name as Route) {
    case "deck": {
      await commands.exportDecks();
      break;
    }
    case "trunk": {
      await commands.exportTrunk();
      break;
    }
  }
}
</script>

<template>
  <main class="fixed inset-0 select-none pb-safe">
    <Sonner :position="md ? 'bottom-right' : 'top-center'" />
    <div v-if="isDev" class="fixed bottom-1 right-1 z-50">
      <span class="text-red-500 font-extrabold">{{ `v${version} DEV` }}</span>
    </div>

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
          <Button variant="outline" :disabled="locked" @click="commands.exportDatabaseFile">
            <DatabaseBackupIcon class="size-6" />
          </Button>
          <Button variant="outline" :disabled="locked || !canExportJson" @click="exportJson">
            <FileInputIcon class="size-6" />
          </Button>
          <Button variant="outline" @click="commands.openSettingsFile">
            <SettingsIcon class="size-6" />
          </Button>
        </div>
      </div>

      <div class="relative size-full overflow-hidden">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <Suspense>
              <component :is="Component" />
              <template #fallback>
                <Loading class="absolute inset-0" />
              </template>
            </Suspense>
          </template>
        </RouterView>
      </div>
    </div>
  </main>
</template>
