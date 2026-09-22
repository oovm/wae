<script setup lang="ts">
import { useWae } from "@wae/adapter-vue";
import { ref } from "vue";

const wae = useWae();
const label = ref("WAE + Vue");
function ping() {
  label.value = `client ok · baseUrl=${wae.server ? "set" : "?"}`;
}
</script>

<template>
  <main style="font-family: system-ui; padding: 2rem">
    <h1>{{ label }}</h1>
    <button type="button" @click="ping">ping client</button>
  </main>
</template>
