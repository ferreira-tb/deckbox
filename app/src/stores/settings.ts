import { ref } from "vue";
import { defineStore } from "pinia";
import { useToggle } from "@vueuse/core";
import type { Option } from "@tb-dev/utils";
import {
  SETTINGS_BACKUP_DIR,
  SETTINGS_CAN_EDIT,
  SETTINGS_CHECK_TRUNK,
  SETTINGS_STORE_ID,
  SETTINGS_TRUNK_DIR,
} from "@/lib/bindings";

export const useSettings = defineStore(SETTINGS_STORE_ID, () => {
  const backupDir = ref<Option<string>>();
  const trunkDir = ref<Option<string>>();

  const [canEdit, toggleEdit] = useToggle(true);
  const [checkTrunk, toggleTrunkCheck] = useToggle(true);

  return {
    [SETTINGS_BACKUP_DIR]: backupDir,
    [SETTINGS_CAN_EDIT]: canEdit,
    [SETTINGS_CHECK_TRUNK]: checkTrunk,
    [SETTINGS_TRUNK_DIR]: trunkDir,
    toggleEdit,
    toggleTrunkCheck,
  };
});
