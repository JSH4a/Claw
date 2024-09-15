<template>
  <tr class="header-row">
    <th class="header-icon">
    </th>
    <th :class="`header-name ` + sortClass(ResultSortOption.NAME)" @click="updateSortType(ResultSortOption.NAME)">
      Name
    </th>
    <th :class="`header-date ` + sortClass(ResultSortOption.DATE)" @click="updateSortType(ResultSortOption.DATE)">
      Last modified
    </th>
    <th :class="`header-type ` + sortClass(ResultSortOption.TYPE) " @click="updateSortType(ResultSortOption.TYPE)">
      Type
    </th>
  </tr>
</template>
<script>

import { ResultSortOption, ResultSortType } from "@/components/results/ResultSortType";

export default {
  computed: {
    ResultSortOption() {
      return ResultSortOption
    },
  },
  data() {
    return {
      sortOption: ResultSortOption.TYPE,
      sortType: ResultSortType.ASC
    }
  },
  methods: {
    sortClass(option) {
      if (option !== this.sortOption) {
        return;
      }
      return this.sortType;
    },
    updateSortType(option) {
      if (option === this.sortOption) {
        this.toggleSortType();
      } else {
        this.sortType = ResultSortType.DSC;
      }
      this.sortOption = option;

      this.resultSortTypeEmitter.emit("result-sort-type-update",
          {sortOption: this.sortOption, sortType: this.sortType});
    },
    toggleSortType() {
      if (this.sortType === ResultSortType.ASC) {
        this.sortType = ResultSortType.DSC;
      } else {
        this.sortType = ResultSortType.ASC;
      }
    },
  },
}
</script>

<style scoped>
.header-row {
  height: 100%;
  width: 100%;
  table-layout: fixed;
  text-align: left;

  display: flex;
  align-items: center;
  user-select: none;

  position: sticky;
  top: 0px;
  z-index: 1;
}

tr {
  background: black;
  line-height: 2rem;
}
th {
  margin-top: 1rem;
  height: 2rem;
  background: black;
}

.header-icon {
  margin-left: 1.5rem;
}

.header-name {
  margin-left: 1rem;
  width: 50%;
}

.header-date {
  margin-left: auto;
  width: 15%;
}

.header-type {
  margin-left: 1rem;
  margin-right: 1rem;
  width: 10%;
}

.ASC::after {
  content: "\25B2"; /* Unicode for up arrow */
  color: white;      /* Set the color of the arrow */
}

.DSC::after {
  content: "\25BC"; /* Unicode for up arrow */
  color: white;      /* Set the color of the arrow */
}
</style>