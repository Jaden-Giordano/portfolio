<template>
  <page>
    <div class="home">
      <span class="title">Jaden Giordano</span>
      <hr>
      <div class="tags">
        <span>Frontend</span>
        <span>Game Development</span>
        <span>Design</span>
      </div>
    </div>
  </page>
</template>

<script>
  import Page from '@/components/page';

  export default {
    components: {
      Page,
    },
  };
</script>

<style lang="scss">
  .home {
    align-items: center;
    display: flex;
    flex-direction: column;
    height: 100%;
    justify-content: center;
    width: 100%;

    .title {
      color: white;
      font-family: "Rock Salt";
      font-size: 2rem;
    }

    hr {
      border-color: white;
      width: 30%;
    }

    .tags {
      color: white;
      display: flex;
      font-family: Roboto;
      font-size: 1rem;
      margin-top: 0.75rem;

      span {
        margin-right: 0.75rem;

        &:nth-child(1) {
          color: #1199ee;
        }

        &:nth-child(2) {
          color: #66bb66;
        }

        &:nth-child(3) {
          color: #ffaa22;
        }

        &::after {
          color: white;
          margin-left: 0.5rem;
          content: ' \2022';
        }

        &:last-child {
          margin-right: 0;

          &::after {
            margin-left: 0;
            content: '';
          }
        }
      }
    }
  }
</style>
