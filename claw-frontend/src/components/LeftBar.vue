<template>
<div class="left-bar">
  <ul>
    <li> Directory 1 </li>
    <li> Directory 2 </li>
    <li> Directory 3 </li>
    <li> Directory 4 </li>
    <li> Directory 5 </li>
  </ul>
</div>
</template>

<script>
export default {
  name: 'left-bar',
}
</script>

<style scoped>
.left-bar {
  border: 2px solid white;
  border-radius: 30px;
  overflow: auto;
  margin: 30px 50px 0px 50px;

  width: 15rem;
  height: 10rem;
  padding: 20px;
  text-align: left;

  float: left;
}

ul {
  list-style: none;
  margin-left: 0;
}

li {
  margin-top: 1em;
  margin-bottom: 1em;
}

li:before {
  content: "📁";
  padding-right: 5px;
}
</style>