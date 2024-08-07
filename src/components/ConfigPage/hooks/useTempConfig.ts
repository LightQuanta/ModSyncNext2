import { computed, watch } from "vue"
import { useConfigStore } from "@/store/config"
import { useCloned } from '@vueuse/core'

export default function useTempConfig() {
  const configStore = useConfigStore()
  const {
    cloned: tempConfig,
    // 这个 sync 是从 store 同步配置到 tempConfig，而不是 temp 到 store
    sync
    // 使用函数来保证每次组件更新时监听的是 store 内的值，如果直接传值，由于 configStore.config 取出来的是源对象，所以没法触发更新
  } = useCloned(() => configStore.config)

  /**
   * 将 tempConfig 同步到 store
   */
  const syncConfigToStore = () => {
    // 创建一个全新对象，如果直接使用 tempConfig，由于深度的对象仍为 proxy 对象，会导致后续修改时发生响应式变化
    configStore.config = JSON.parse(JSON.stringify(tempConfig.value))
    // TODO 在这里实现 tauri invoke 配置保存
  }

  watch(() => configStore.config, () => {
    sync()
  })

  const tempConfigJSON = computed(() => JSON.stringify(tempConfig.value, null, 4))
  const configJSON = computed(() => JSON.stringify(configStore.config, null, 4))

  // 对比字符串，如果内容多了性能可能变差
  const changed = computed(() => tempConfigJSON.value !== configJSON.value)

  return {
    changed,
    tempConfig,
    syncConfigToStore,
    tempConfigJSON,
    configJSON,
  }
}