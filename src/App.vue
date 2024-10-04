<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core"
import { Config } from "@/config"
import { ElMessage } from "element-plus"
import { useConfigStore } from "@/store/config"

import Main from "@/components/Main.vue";
import ConfigPage from "@/components/ConfigPage/index.vue";


onMounted(async () => {
  const configStore = useConfigStore()
  try {
    if (await invoke("has_config")) {
      const configText: string = await invoke("read_config");
      if (configText.startsWith("error:")) {
        throw new Error(configText)
      }
      const config: Config = JSON.parse(configText)
      configStore.config = config
    } else {
      // TODO 实现默认配置创建
      ElMessage({
        type: "info",
        message: `未发现配置文件，等待创建`,
        showClose: true,
        duration: 0,
      })

    }
  } catch (error) {
    ElMessage({
      type: "error",
      message: `加载配置出错：${error}\n\n正在使用默认配置`,
      showClose: true,
      duration: 0,
    })
  }
})

const configChanged = ref(false)
</script>

<template>
  <ElTabs class="w-full" :stretch="true">
    <ElTabPane label="Main">
      <Main></Main>
    </ElTabPane>
    <ElTabPane :label="'设置' + (configChanged ? '*' : '')">
      <ConfigPage @changed="changed => configChanged = changed" />
    </ElTabPane>
  </ElTabs>
</template>

<style scoped>
.el-tabs :deep(.el-tabs__header) {
  margin-bottom: 0;
}
</style>