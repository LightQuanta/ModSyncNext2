<script setup lang="ts">
import { ref } from "vue"
import { ElAutocomplete, ElButton, ElInput, ElCheckbox, ElSelect, ElOption, ElForm, ElFormItem } from "element-plus";
import { ActionAfterSync } from "@/config"
import useTempConfig from './hooks/useTempConfig'

const {
    tempConfig,
    tempConfigJSON,
    configJSON,
    changed,
    syncConfigToStore,
} = useTempConfig()

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
    temp:
    <pre>{{ tempConfigJSON }}</pre>
    store:
    <pre>{{ configJSON }}</pre>
    <p>{{ changed }}</p>
    <ElForm label-width="auto">
        <ElFormItem label="同步服务器网址">
            <ElInput v-model="tempConfig.sync.server" placeholder="同步服务器"></ElInput>
        </ElFormItem>
        <ElFormItem label="更新选项">
            <ElCheckbox v-model="tempConfig.sync.autoSync">自动同步</ElCheckbox>
            <ElCheckbox v-model="tempConfig.sync.autoUpdate">自动更新</ElCheckbox>
        </ElFormItem>
        <ElFormItem label="同步后行为">
            <ElSelect v-model="tempConfig.sync.actionAfterSync" placeholder="同步后行为">
                <ElOption v-for="o in ActionAfterSync" :key="o" :label="o" :value="o" />
            </ElSelect>
        </ElFormItem>

        <ElFormItem label="同步后执行命令">
            <ElInput v-model="tempConfig.sync.command" placeholder="同步后执行命令"></ElInput>
        </ElFormItem>

        <ElFormItem label="要同步的Minecraft版本">
            <ElAutocomplete v-model="tempConfig.minecraft.version" :fetch-suggestions="querySearch" clearable
                placeholder="要同步的Minecraft版本"></ElAutocomplete>
        </ElFormItem>
        <ElFormItem label="同步设置">
            <ElCheckbox v-model="tempConfig.minecraft.isolate">版本隔离</ElCheckbox>
            <ElCheckbox v-model="tempConfig.minecraft.syncConfig">同步配置文件</ElCheckbox>
        </ElFormItem>
        <ElFormItem label="操作">
            <ElButton @click="syncConfigToStore">保存</ElButton>
            <!-- <ElButton @click="configStore.config = tempConfig">恢复默认设置</ElButton> -->
        </ElFormItem>
    </ElForm>
</template>
