<template>
  <page>
    <div slot="main" class="header">
      <span class="title">Aidan Chelig</span>
      <hr />
      <div class="tags">
        <span>Frontend</span>
        <span>Game Development</span>
        <span>Design</span>
      </div>
    </div>
    <template #secondary>
      <h1 class="title">
        What I've been working on...
      </h1>
      <div class="projects">
        <project
          title="Something"
          content="We did some really cool stuff this one time using this one thing and this other thing. It was really cool."
        >
          <rust-icon slot="icon" class="project__icon" />
          <template #links>
            <a class="link-button" href="#">Source</a>
            <a class="link-button" href="#">Demo</a>
          </template>
        </project>
        <project
          title="Something"
          content="We did some really cool stuff this one time using this one thing and this other thing. It was really cool."
        >
          <rust-icon slot="icon" class="project__icon" />
          <template #links>
            <a class="link-button" href="#">Source</a>
            <a class="link-button" href="#">Demo</a>
          </template>
        </project>
        <project
          title="Something"
          content="We did some really cool stuff this one time using this one thing and this other thing. It was really cool."
        >
          <rust-icon slot="icon" class="project__icon" />
          <template #links>
            <a class="link-button" href="#">Source</a>
            <a class="link-button" href="#">Demo</a>
          </template>
        </project>
      </div>
    </template>
    <template #footer>
      <span class="footer__text">2020 &copy; Aidan Chelig</span>
      <div class="contact">
        <a class="nav-link" href="mailto:aidan.chelig@gmail.com">Email</a>
        <a class="nav-link" href="https://github.com/Aidan-Chelig">Github</a>
        <a class="nav-link" href="https://www.linkedin.com/in/aidan-chelig-9a296613a/">LinkedIn</a>
        <a class="nav-link" href="https://drive.google.com/file/d/1G3cGu572gyr7cMqneHjFX6rSq7tB9f-S/view?usp=sharing">
          Resume
        </a>
      </div>
    </template>
  </page>
</template>

<script>
import Page from "@/components/page";
import Project from '@/components/project';
import RustIcon from '@/components/icons/rust';

export default {
  components: {
    Page,
    Project,
    RustIcon
  },
};
</script>

<style lang="scss">
.contact {
  color: white;
  display: flex;
  justify-content: space-between;
}

.title {
  color: white;
  font-family: "Rock Salt";
  font-size: 2rem;
}

.header {
  align-items: center;
  display: flex;
  flex-direction: column;
  height: 100%;
  justify-content: center;

  hr {
    border-color: white;
    width: 30%;
  }

  .tags {
    color: white;
    display: flex;
    font-family: Roboto;
    font-weight: 300;
    font-size: 1rem;
    text-transform: uppercase;
    letter-spacing: 0.075rem;
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
        content: " \2022";
      }

      &:last-child {
        margin-right: 0;

        &::after {
          margin-left: 0;
          content: "";
        }
      }
    }
  }
}

.projects {
  display: flex;
  flex-wrap: wrap;

  & > * {
    margin-right: 2rem;

    &:last-child {
      margin-right: 0;
    }
  }
}
</style>
