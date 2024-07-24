import { defineStore } from "pinia";
import { ref } from "vue";
import { Config, ActionAfterSync } from "@/config.ts"

export const useConfigStore = defineStore('config', () => {
    const config = ref<Config>({
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
    })

    return { config }
})