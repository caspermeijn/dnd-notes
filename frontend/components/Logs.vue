<template>
  <div class="logs">
    <h1 class="text-4xl roboto font-extrabold">Logs</h1>
    <Log v-for="log in logs" :log="log"/>
  </div>
</template>

<script setup>

const {data: logs} = await useApi('logs');

</script>

<style scoped lang="scss">

.logs {
  margin: 1em 0 0 1em;
}

:deep(.log) {
  +.log {
    margin-top: 0.5em;
  }
}
</style>