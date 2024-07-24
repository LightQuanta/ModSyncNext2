<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri"
import { Config } from "@/components/config"

// TODO 实现主界面

const greetMsg = ref("")
const path = ref("")

async function greet() {
  const config: Config = JSON.parse(await invoke("greet", { path: path.value }))
  greetMsg.value = JSON.stringify(config, null, 4)
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <p>Input config path</p>
    <input id="greet-input" v-model="path" placeholder="Input config path" />
    <button type="submit">Greet</button>
  </form>
  <pre>{{ greetMsg }}</pre>
</template>
