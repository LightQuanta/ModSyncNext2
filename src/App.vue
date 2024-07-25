<script setup lang="ts">
import { onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri"
import { Config } from "@/config"
import { ElMessage } from "element-plus"
import { useConfigStore } from "@/store/config"

import Main from "@/components/Main.vue";


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

// TODO 实现正确的配置处理
const tempConfigJSON = computed(() => JSON.stringify(useConfigStore().config, null, 4))

</script>

<template>
  <el-tabs class="w-full" stretch="true">
    <el-tab-pane label="Main">
      <Main></Main>
    </el-tab-pane>
    <el-tab-pane label="Config">
      <!-- 临时Config预览 -->
      <pre>{{ tempConfigJSON }}</pre>
    </el-tab-pane>
  </el-tabs>
</template>
