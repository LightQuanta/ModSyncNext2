<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const path = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { path: path.value });
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <p>Input config path</p>
    <input id="greet-input" v-model="path" placeholder="Input config path" />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
