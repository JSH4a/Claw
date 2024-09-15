<template>
  <div class="results-table-wrapper">
    <table class="results-table">
      <result-header-row/>

      <result-row v-for="result in sortedResults" :key="result.name"
                   :path="result.path"
                   :name="result.name"
                   :date-modified="result.last_modified"
                   :type="result.file_type">
        <directory-icon v-if="result.file_type === 'Directory'"/>
        <generic-f-ile-icon v-else/>
      </result-row>

    </table>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import ResultRow from "@/components/results/ResultRow.vue";
import ResultHeaderRow from "@/components/results/ResultHeaderRow.vue";
import DirectoryIcon from "@/components/results/icons/DirectoryIcon.vue";
import GenericFIleIcon from "@/components/results/icons/GenericFileIcon.vue";
import {ResultSortOption, ResultSortType} from "@/components/results/ResultSortType";

export default defineComponent({
  components: {GenericFIleIcon, DirectoryIcon, ResultHeaderRow, ResultRow },
  props: {
    results: JSON,
  },
  data() {
    return {
      sortOption: ResultSortOption.TYPE,
      sortType: ResultSortType.ASC,
    }
  },
  mounted() {
    this.resultSortTypeEmitter.on('result-sort-type-update', (newType) => {
      this.sortType = newType.sortType;
      this.sortOption = newType.sortOption;
    });
  },
  computed: {
    sortedResults() {
      if (!Array.isArray(this.results)) {
        return []; // Return an empty array if results is not an array
      }

      let sortedResults;
      switch (this.sortOption) {
        case ResultSortOption.NAME:
          sortedResults = this.results.slice().sort((a, b) => this.sortByName(a, b));
          break;
        case ResultSortOption.DATE:
          sortedResults = this.results.slice().sort((a, b) => this.sortByDate(a, b));
          break;
        case ResultSortOption.TYPE:
          sortedResults = this.results.slice().sort((a, b) => this.sortByType(a, b));
          break;
      }

      if (this.sortType === ResultSortType.DSC) {
        return sortedResults.reverse();
      }
      return sortedResults;
    },
  },
  methods: {
    sortByName(a, b) {
      const nameComparison = a.name.localeCompare(b.file_type);
      if (nameComparison !== 0) {
        return nameComparison;
      }
      return a.name.localeCompare(b.name);
    },
    sortByDate(a, b) {
      return a.last_modified > b.last_modified? 1 : -1;
    },
    sortByType(a, b) {
      const typeComparison = a.file_type.localeCompare(b.file_type);
      if (typeComparison !== 0) {
        return typeComparison;
      }
      return a.name.localeCompare(b.name);
    },
  }
});
</script>

<style>
.results-table-wrapper {
  margin-left: 1rem;
  margin-right: 1rem;
  margin-top: 2rem;
  border: 2px solid white;
  border-radius: 30px;
  padding: 1rem;

  height: 50rem;
  overflow: scroll;
}

.results-table {
  transform: translateY(-16px);

  width: 98%;
  padding: 1rem;

  table-layout: fixed;

  border-collapse: collapse;

  user-select: none;
}
</style>