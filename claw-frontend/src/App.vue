<!-- App.vue -->
<template>
<link rel="stylesheet" href="@/assets/static/stylesheets/global.css">

  <input type="text" v-model="name">
  <button @click="say_hello">say hi</button>
  <p>{{ greeting }}</p>
</template>

<script>
import { invoke } from '@tauri-apps/api'

export default {
  data() {
    return {
      greeting: '',
      name: ''
    }
  },
  name: 'App',
  components: {
  },
  methods: {
    say_hello() {
    invoke('read_directory', { directoryPath: this.name }).then((response) => {
      this.greeting = response;
    });
  },

  }
}



</script>

<style>
* {
  background: black;
  color: white;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>