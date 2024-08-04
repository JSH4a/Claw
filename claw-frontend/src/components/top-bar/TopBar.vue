<template>
<div class="top-bar">
  <input class="search-input"
         type="text"
         v-model="path"
         placeholder="Enter directory or search..."
         @keydown.enter.prevent="this.readDirectory">

  <claw-button class="claw-button" @click.prevent="this.readDirectory"/>
</div>
</template>

<script>
import { invoke } from '@tauri-apps/api';
import ClawButton from "@/components/top-bar/ClawButton.vue";
export default {
  components: {
    ClawButton
  },
  data() {
    return {
      path: '',
    }
  },
  mounted() {
    this.pathEmitter.on('path-update', (newPath) => {
      console.log(newPath);
      this.path = newPath;
      this.readDirectory();
    });
  },
  methods: {
    readDirectory() {
      invoke('read_directory', { directory_path: this.path }).then((response) => {
        this.$emit("update-results", JSON.parse(response.toString()));
      });
    },
  }
}
</script>

<style scoped>
.top-bar {
  display: flex;
  justify-content: space-evenly;

  border: 2px solid white;
  border-radius: 30px;
  overflow: hidden;
  margin-left: 1rem;
  margin-right: 1rem;
  margin-top: -2rem;
  height: 3rem;
}

.search-input {
  flex-grow: 1;
  padding: 0.5rem;
  margin-left: 0.6rem;
  border: none;
  outline: none;
  box-sizing: border-box;
}

.claw-button {
  flex-grow: 0;
  border: none;
  background: white;
  width: 10%;

  height: 3rem;
}
</style>