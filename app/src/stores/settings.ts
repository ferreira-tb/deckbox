import { ref } from "vue";
import { defineStore } from "pinia";
import { useToggle } from "@vueuse/core";
import type { Option } from "@tb-dev/utils";
import { SETTINGS_BACKUP_DIR, SETTINGS_CAN_EDIT, SETTINGS_TRUNK_DIR } from "@/lib/bindings";

export const useSettings = defineStore("settings", () => {
  const backupDir = ref<Option<string>>();
  const trunkDir = ref<Option<string>>();

  const [canEdit, toggleEdit] = useToggle(true);

  return {
    [SETTINGS_BACKUP_DIR]: backupDir,
    [SETTINGS_CAN_EDIT]: canEdit,
    [SETTINGS_TRUNK_DIR]: trunkDir,
    toggleEdit,
  };
});
