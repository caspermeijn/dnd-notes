<template>
  <div class="input">
    <form @submit="submitLog">
      <input type="text" class="border-2 text" id="log" placeholder="Log text" ref="log" required/>
      <input type="text" class="border-2 text" id="tags" placeholder="Tags (comma separated)" ref="tags" required/>
      <input type="submit" class="submit rounded-full shadow bg-slate-600 text-slate-50" value="Submit"/>
    </form>
  </div>
</template>

<script setup>

const log = ref(null);
const tags = ref(null);

const submitLog = async (e) => {
    e.preventDefault();

    const logText = log.value.value;
    const tagsText = tags.value.value.split(',').map(t => t.trim());

    const result = await useApi('logs', {
      method: 'POST',
      body: {
        text: logText,
        tags: tagsText
      }
    });

    console.log(result);
};

</script>

<style scoped lang="scss">
.input {
  .text {
    display: block;
    margin-bottom: 0.4em;
    padding: 1em 0.5em;
    width: 100%;
    box-sizing: border-box;
  }

  .submit {
    display: inline-block;
    width: 75px;
    height: 75px;
    line-height: 75px;
    text-align: center;
  }
}
</style>