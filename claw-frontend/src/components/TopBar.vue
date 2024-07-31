<template>
<div class="top-bar">
  <img class="title-image" src="@/assets/logo.png">
  <input type="text" v-model="name">
    <button @click="say_hello">say hi</button>
    <p>{{ greeting }}</p>
</div>
</template>

<script>
import { invoke } from '@tauri-apps/api';

export default {
  data() {
      return {
        greeting: '',
        name: ''
      }
  },
  name: 'TopBar',
  components: {
  },
  methods: {
    say_hello() {
      invoke('read_directory', { directory_path: this.name }).then((response) => {
      this.greeting = response;
      });
    },
  }
}
</script>

<style scoped>
.top-bar {
  border: 2px solid white;
  border-radius: 30px;
  overflow: auto;
  margin: 0px 50px 0px 50px;
}

.title-image {
  height: 4rem;
  float: left;
  margin-left: 2rem;
}
</style>