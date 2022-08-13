<template>
  <div class="index">
    <h1 class="text-4xl roboto font-extrabold">Tags</h1>
    <Tags class="tags" :selectable="true"/>

    <h1 class="text-4xl roboto font-extrabold">Logs</h1>
    <Logs :filter="state.tags" :data="logs"/>
  </div>
</template>
<script setup>

const state = reactive({
  tags: []
});

const {data: logs} = await useApi('logs');

useMitt().on('tag', (e) => {

  if (!e.selected) {
    state.tags.splice(state.tags.indexOf(e.value), 1);
  } else {
    state.tags.push(e.value);
  }
});

</script>

<style scoped lang="scss">
.index {

  margin: 1em 0 0 1em;

  h1 {
    margin-bottom: 0.3em;
  }

  .tags {
    margin-bottom: 2em;
    overflow: scroll;
    white-space: nowrap;

    -ms-overflow-style: none;  /* Internet Explorer 10+ */
    scrollbar-width: none;  /* Firefox */

    &::-webkit-scrollbar {
      display: none;
    }
  }
}
</style>