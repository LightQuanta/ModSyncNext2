<script setup lang="ts">
import { ref, watchEffect } from "vue"
import {
    ElAutocomplete,
    ElButton,
    ElInput,
    ElCheckbox,
    ElSelect,
    ElOption,
    ElForm,
    ElFormItem,
    ElMessage,
    ElMessageBox,
} from "element-plus";
import { ActionAfterSync } from "@/config"
import useTempConfig from './hooks/useTempConfig'
import { invoke } from "@tauri-apps/api/tauri"

const {
    tempConfig,
    changed,
    syncConfigToStore,
    discardChanges,
    resetConfig,
} = useTempConfig()

const emit = defineEmits<{
    changed: [changed: boolean]
}>()

watchEffect(() => emit("changed", changed.value))

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

const saveConfig = async () => {
    const result = await syncConfigToStore()
    if (result === "ok") {
        ElMessage({
            message: "保存成功",
            type: "success",
        })
    } else {
        ElMessage({
            message: "保存失败：" + result,
            type: "error",
        })
    }
}

const onDiscardChanges = () => {
    ElMessageBox.confirm(
        "确定要取消更改吗？",
        "提示",
        {
            confirmButtonText: "确定",
            cancelButtonText: "取消",
            type: "warning",
        }
    ).then(discardChanges)
}

const onResetConfig = () => {
    ElMessageBox.confirm(
        "确定要恢复默认配置吗？",
        "提示",
        {
            confirmButtonText: "确定",
            cancelButtonText: "取消",
            type: "warning",
        }
    ).then(async () => {
        resetConfig()
        saveConfig()
    })
}

const getActionDisplayName = (action: ActionAfterSync) => {
    switch (action) {
        case ActionAfterSync.Exit: return "退出程序"
        case ActionAfterSync.DoNothing: return "待命"
        case ActionAfterSync.ExecuteCommand: return "执行命令"
        case ActionAfterSync.ExecuteCommandAndExit: return "执行命令并退出"
    }
}

const chooseFile = async () => {
    const path: string | null = await invoke("choose_file")
    if (path !== null) {
        tempConfig.value.sync.command = path
    }
}
</script>

<template>
    <ElForm label-width="auto" class="m-4">
        <ElFormItem label="同步服务器网址">
            <ElInput v-model="tempConfig.sync.server" placeholder="同步服务器"></ElInput>
        </ElFormItem>
        <ElFormItem label="更新选项">
            <ElCheckbox v-model="tempConfig.sync.autoSync">自动同步</ElCheckbox>
            <ElCheckbox v-model="tempConfig.sync.autoUpdate">自动更新</ElCheckbox>
        </ElFormItem>
        <ElFormItem label="同步后行为">
            <ElSelect v-model="tempConfig.sync.actionAfterSync" placeholder="同步后行为">
                <ElOption v-for="action in ActionAfterSync" :key="action" :label="getActionDisplayName(action)"
                    :value="action" />
            </ElSelect>
        </ElFormItem>

        <ElFormItem label="同步后执行命令">
            <div class="flex flex-row grow gap-1">
                <ElInput class="grow" v-model="tempConfig.sync.command" placeholder="同步后执行命令" clearable
                    :disabled="tempConfig.sync.actionAfterSync === 'Exit' || tempConfig.sync.actionAfterSync === 'DoNothing'" />
                <ElButton type="primary" @click="chooseFile"
                    :disabled="tempConfig.sync.actionAfterSync === 'Exit' || tempConfig.sync.actionAfterSync === 'DoNothing'">
                    浏览</ElButton>
            </div>
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
            <ElButton type="primary" @click="saveConfig">保存</ElButton>
            <ElButton class="mr-auto" @click="onDiscardChanges" :disabled="!changed">取消更改</ElButton>
            <ElButton type="danger" @click="onResetConfig">恢复默认配置</ElButton>
        </ElFormItem>
    </ElForm>
</template>
