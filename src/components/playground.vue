<template>
  <canvas :id="id" />
</template>

<script>
  import uid from 'uid';

  export default {
    setup: () => ({
      id: uid(8),
    }),
  }
</script>
