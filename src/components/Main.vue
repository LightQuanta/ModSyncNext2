<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri"
import { Config } from "@/config"
import useTempConfig from '@/components/ConfigPage/hooks/useTempConfig'

// TODO 实现主界面

const {
  tempConfig
} = useTempConfig()
const resp = ref("")

async function greet() {

  resp.value = await invoke("get_mods_info", { version: tempConfig.value.minecraft.version })
}
</script>

<template>
  <form class="row" @submit.prevent>
    <button @click="greet()" type="submit">Greet</button>
  </form>
  <pre>{{ tempConfig.minecraft.version }}</pre>
  <pre>{{ resp }}</pre>
</template>
