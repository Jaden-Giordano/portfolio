<template>
  <div class="playground">
    <canvas :id="id" />
  </div>
</template>

<script>
  import uid from 'uid';

  export default {
    setup: () => ({
      id: uid(8),
    }),
  }
</script>

<style lang="scss">
  .playground {
    height: 100%;
    position: absolute;
    width: 100%;
  }
</style>
