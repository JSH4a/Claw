<!-- App.vue -->
<template>
<link rel="stylesheet" href="@/assets/static/stylesheets/global.css">
  <div id="app">
    <top-bar @update-results="updateResults"/>
    <results-container :results="this.results"/>
  </div>
</template>

<script>
import TopBar from '@/components/top-bar/TopBar.vue';
import ResultsContainer from "@/components/results/ResultsContainer.vue";

export default {
  data() {
    return {
      results: JSON,
    }
  },
  name: 'App',
  components: {
    ResultsContainer,
    'top-bar': TopBar,
  },
  methods: {
    updateResults(newResults) {
      this.results = newResults;
    }
  },
}
</script>

<style>
* {
  background: black;
  color: white;
  font-size: 1.1rem;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 35px;

  display: flex;
  flex-direction: column;
}
</style>