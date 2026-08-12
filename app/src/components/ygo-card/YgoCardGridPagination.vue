<script setup lang="ts">
import { Button, Pagination, PaginationEllipsis, PaginationList, PaginationListItem } from "@tb-dev/vue-components";

interface Props {
  itemsPerPage: number;
  total: number;
}

defineProps<Props>();

const currentPage = defineModel<number>("page", { required: true });
</script>

<template>
  <Pagination
    #default="{ page }"
    v-model:page="currentPage"
    :total
    :items-per-page
    :sibling-count="3"
    show-edges
  >
    <PaginationList #default="{ items }" class="flex items-center gap-1">
      <template v-for="item of items">
        <PaginationListItem
          v-if="item.type === 'page'"
          :key="item.value"
          :value="item.value"
          as-child
        >
          <Button
            :variant="item.value === page ? 'default' : 'outline'"
            size="sm"
            class="size-8 p-0"
          >
            <span>{{ item.value }}</span>
          </Button>
        </PaginationListItem>
        <PaginationEllipsis v-else :key="item.type" />
      </template>
    </PaginationList>
  </Pagination>
</template>
