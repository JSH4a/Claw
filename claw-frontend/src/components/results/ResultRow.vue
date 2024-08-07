<template>
  <tr class="result-row" @dblclick.prevent="updatePath">
    <td>
      <div class="result-icon">
        <slot/>
      </div>
    </td>
    <td class="result-name">
      <p v-text="name"/>
    </td>
    <td class="result-date">
      <p v-text="dateModified"/>
    </td>
    <td class="result-type">
      <p v-text="type"/>
    </td>
  </tr>
</template>
<script>

export default {
  props: {
    path: String,
    name: String,
    dateModified: Date,
    type: String,
  },
  methods: {
    updatePath() {
      if (this.type === "Directory") {
        this.pathEmitter.emit("path-update", this.path);
      }
    },
  }
}
</script>

<style>
.result-row {
  table-layout: fixed;
  text-align: left;

  display: flex;
  align-items: center;
}
.result-row:active {
  outline: 1px solid red;
  border-radius: 30px;
  outline-offset: 0.5rem;
  z-index: 10;
}

.result-icon {
  transform: translateY(2px);
  width: 1.5rem;
}

.result-name {
  margin-left: 1rem;
  width: 50%;
}

.result-date {
  margin-left: auto;
  width: 15%;
}

.result-type {
  margin-left: 1rem;
  width: 10%;
}
</style>