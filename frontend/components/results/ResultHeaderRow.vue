<template>
  <tr class="header-row">
    <th class="header-icon">
    </th>
    <th class="header-name">
      Name
    </th>
    <th class="header-date">
      Last modified
    </th>
    <th class="header-type">
      Type
    </th>
  </tr>
</template>
<script>

export default {
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
</style>