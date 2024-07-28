<!-- App.vue -->
<template>
  <input type="text" v-model="name">
  <button @click="say_hello">say hello</button>
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
    invoke('greet', { name: this.name }).then((response) => {
      this.greeting = response;
    });
  },

  }
}



</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>