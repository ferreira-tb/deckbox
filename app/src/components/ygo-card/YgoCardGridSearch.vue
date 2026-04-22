<script setup lang="ts">
import { useTemplateRef } from 'vue';
import { SearchIcon } from '@lucide/vue';
import type { Option } from '@tb-dev/utils';
import { Input } from '@tb-dev/vue-components';

const searchValue = defineModel<Option<string>>({ required: true });
const searchInput = useTemplateRef('searchInputEl');

defineExpose({
  // eslint-disable-next-line
  focus: () => searchInput.value?.$el?.focus(),
});
</script>

<template>
  <div class="w-68">
    <div class="relative flex w-full items-center">
      <Input
        ref="searchInputEl"
        v-model="searchValue"
        type="text"
        autocapitalize="off"
        autocomplete="off"
        spellcheck="false"
        class="w-full pl-10"
        @keydown.stop
      />
      <span class="absolute inset-y-0 inset-s-0 flex items-center justify-center px-2">
        <SearchIcon class="text-muted-foreground size-6" />
      </span>
    </div>
  </div>
</template>
