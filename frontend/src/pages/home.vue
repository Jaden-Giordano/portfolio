<template>
  <page>
    <div slot="main" class="header">
      <span class="title glow">{{ name }}</span>
      <hr />
      <div class="tags">
        <span v-for="tag in tags" :key="tag">{{tag}}</span>
      </div>
    </div>
    <template #secondary>
      <h1 class="title glow">
        {{projectsTitle}}
      </h1>
      <div class="projects">
        <project
          v-for="project in projects"
          :key="project.name"
          :title="project.name"
          :content="project.description"
        >
          <project-icon :lang="project.language" />
          <template #links>
            <a
              v-for="link in project.links"
              :key="link.name"
              :href="link.href"
              class="link-button"
            >
              {{link.name}}
            </a>
          </template>
        </project>
      </div>
    </template>
    <template #footer>
      <span class="footer__text">2021 &copy; {{name}}</span>
      <div class="contact">
        <a class="nav-link" v-for="link in externalLinks" :key="link.name" :href="link.href">
          {{link.name}}
        </a>
      </div>
    </template>
  </page>
</template>

<script>
import config from '@/config';
import Page from "@/components/page";
import Project from '@/components/project';
import ProjectIcon from '@/components/project-icon';

export default {
  components: {
    Page,
    Project,
    ProjectIcon,
  },
  setup: () => (config || {}),
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
  align-items: flex-start;

  & > * {
    margin-right: 2rem;

    &:last-child {
      margin-right: 0;
    }
  }
}
</style>
