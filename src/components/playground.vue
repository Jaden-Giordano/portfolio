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
