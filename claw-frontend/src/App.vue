<!-- App.vue -->
<template>
<link rel="stylesheet" href="@/assets/static/stylesheets/global.css">
  <top-bar/>
  <left-bar/>
</template>

<script>
import { invoke } from '@tauri-apps/api';
import TopBar from './components/TopBar';
import LeftBar from './components/LeftBar';

export default {
  data() {
    return {
      greeting: '',
      name: ''
    }
  },
  name: 'App',
  components: {
    'top-bar': TopBar,
    'left-bar': LeftBar,
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