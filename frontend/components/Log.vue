<template>
  <div class="log">
    {{log.text}}

    <div class="tags">
      <div class="tag shadow rounded bg-slate-600 text-slate-50" v-for="tag in log.tags">{{tag}}</div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  log: {
    type: [Object]
  }
})
</script>

<style scoped lang="scss">
.log {
  .tags {
    display: inline-block;
    .tag {
      display: inline-block;
      margin-left: 0.5em;

      padding: 0.25em 0.5em;
    }
  }
}
</style>