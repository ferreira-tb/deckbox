<script setup lang="ts">
import type { Route } from '@/router';
import { useRoute } from 'vue-router';
import { NavigationMenuItem, NavigationMenuLink, navigationMenuTriggerStyle } from '@tb-dev/vue-components';

defineProps<{
  route: Route;
  label: string;
}>();

const currentRoute = useRoute();
</script>

<template>
  <NavigationMenuItem>
    <NavigationMenuLink
      as-child
      :active="currentRoute.name === route"
      :class="navigationMenuTriggerStyle()"
    >
      <RouterLink :to="{ name: route }" :data-active="currentRoute.name === route">
        <span>{{ label }}</span>
      </RouterLink>
    </NavigationMenuLink>
  </NavigationMenuItem>
</template>
