<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core"
import { open } from "@tauri-apps/plugin-shell"
import { ElButton, ElCarousel, ElCarouselItem, ElStep, ElSteps } from 'element-plus'
import useTempConfig from '@/components/ConfigPage/hooks/useTempConfig'
import { Download, Delete, List } from '@element-plus/icons-vue'

// TODO 实现主界面

const {
  tempConfig
} = useTempConfig()
const resp = ref("")

async function greet() {
  resp.value = await invoke("get_mods_info", { version: tempConfig.value.minecraft.version })
}

async function fetchVersionInfo() {
  const info = await invoke("fetch_version_info", { version: tempConfig.value.minecraft.version })
  console.log(info)
}

const carousel = ref()
const prev = () => {
  carousel.value.prev()
  index.value--
}
const next = () => {
  carousel.value.next()
  index.value++
}
const index = ref(-1)

const run = async () => {
  await open(tempConfig.value.sync.command)
}
</script>

<template>
  <ElSteps :active="index" finish-status="success" simple>
    <ElStep title="获取同步信息" :icon="List" />
    <ElStep title="删除多余mod" :icon="Delete" />
    <ElStep title="下载缺失mod" :icon="Download" />
  </ElSteps>
  <ElCarousel ref="carousel" :autoplay="false" arrow="never" indicator-position="none" :motion-blur="true">
    <ElCarouselItem class="bg-gray-200">
      开始同步
      <ElButton @click="greet()">Greet</ElButton>
      <pre>{{ tempConfig.minecraft.version }}</pre>
      <pre>{{ resp }}</pre>
    </ElCarouselItem>
    <ElCarouselItem class="bg-yellow-50">
      获取同步信息
      <ElButton @click="fetchVersionInfo()">fetch</ElButton>
    </ElCarouselItem>
    <ElCarouselItem class="bg-red-300">
      删除多余mod
    </ElCarouselItem>
    <ElCarouselItem class="bg-blue-300">
      下载缺失mod
    </ElCarouselItem>
    <ElCarouselItem class="bg-green-200">
      完成
      <ElButton @click="run()">CMD</ElButton>
    </ElCarouselItem>
  </ElCarousel>
  <ElButton @click="prev">prev</ElButton>
  <ElButton @click="next">next</ElButton>
</template>
