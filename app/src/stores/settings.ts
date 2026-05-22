import { ref } from "vue";
import { defineStore } from "pinia";

export const useSettings = defineStore("settings", () => {
  const canEdit = ref(true);

  function toggleEdit() {
    canEdit.value = !canEdit.value;
  }

  return {
    canEdit,
    toggleEdit,
  };
});
