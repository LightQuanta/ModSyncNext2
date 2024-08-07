<script setup lang="ts">
import { computed, ref, watch, onMounted } from "vue"
import { useCloned } from '@vueuse/core'
import { useConfigStore } from "@/store/config"
import { ElAutocomplete, ElButton, ElInput, ElCheckbox, ElSelect, ElOption } from "element-plus";
import { Config, ActionAfterSync } from "@/config"

const configStore = useConfigStore()
// const config = configStore.config

let tempConfig = ref(configStore.config)
let syncConfig = () => {
    console.log(1)
    configStore.config = { ...tempConfig.value }
    console.log(configStore.config)
    // console.log(configStore.config)
}

setTimeout(() => {
    const { cloned, sync } = useCloned(configStore.config)
    tempConfig.value = cloned.value
    syncConfig = sync
    console.log(tempConfig.value)
}, 1000);

onMounted(() => {
})

// const tempConfigJSON = computed(() => JSON.stringify(configStore.config, null, 4))
const tempConfigJSON = computed(() => JSON.stringify(tempConfig.value, null, 4))
const configJSON = computed(() => JSON.stringify(configStore.config, null, 4))

const changed = ref(false)
// TODO 修复配置文件同步问题
watch(() => configStore.config, () => {
    changed.value = true
})

// TODO 实现正确补全
const list = ref([
    { value: "111" },
    { value: "222" },
    { value: "333" },
])
const querySearch = (queryString: string, cb: any) => {
    const results = queryString
        ? list.value.filter(v => v.value.includes(queryString))
        : list.value
    cb(results)
}

// TODO 实现配置界面
</script>

<template>
    <pre>{{ tempConfigJSON }}</pre>
    <pre>{{ configJSON }}</pre>
    <p>{{ changed }}</p>
    <ElInput v-model="tempConfig.sync.server" placeholder="同步服务器"></ElInput>
    <ElCheckbox v-model="tempConfig.sync.autoSync">自动同步</ElCheckbox>
    <ElCheckbox v-model="tempConfig.sync.autoUpdate">自动更新</ElCheckbox>
    <ElSelect v-model="tempConfig.sync.actionAfterSync" placeholder="同步后行为">
        <ElOption v-for="o in ActionAfterSync" :key="o" :label="o" :value="o" />
    </ElSelect>
    <ElInput v-model="tempConfig.sync.command" placeholder="同步后执行命令"></ElInput>

    <ElAutocomplete v-model="tempConfig.minecraft.version" :fetch-suggestions="querySearch" clearable
        placeholder="要同步的Minecraft版本"></ElAutocomplete>
    <ElCheckbox v-model="tempConfig.minecraft.isolate">版本隔离</ElCheckbox>
    <ElCheckbox v-model="tempConfig.minecraft.syncConfig">同步配置文件</ElCheckbox>

    <ElButton @click="syncConfig">保存</ElButton>
    <!-- <ElButton @click="configStore.config = tempConfig">恢复默认设置</ElButton> -->
</template>
