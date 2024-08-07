import { defineStore } from "pinia";
import { ref } from "vue";
import { Config, ActionAfterSync } from "@/config.ts"

const defaultConfig = {
    version: "2.0",
    sync: {
        server: "",
        autoUpdate: false,
        autoSync: false,
        actionAfterSync: ActionAfterSync.DoNothing,
        command: ""
    },
    minecraft: {
        version: "",
        isolate: false,
        syncConfig: false
    }
}

export { defaultConfig }

export const useConfigStore = defineStore('config', () => {
    const config = ref<Config>({ ...defaultConfig })
    const restore = () => {
        config.value = { ...defaultConfig }
    }

    return { config, restoreToDefault: restore }
})