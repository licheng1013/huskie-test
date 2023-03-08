<template>
  <div class="sidebar">
    <div v-for="(tab,i) in tabs" class="tab"
         @click="index = i;emit('onChange',index)"
         :class="i === index?'selectedTab':'unselectedTab'" >
      <div class="tab-text">
      {{ tab }}
      </div>
    </div>
  </div>
</template>

<script setup>
import {ref} from "vue";
import {tabs} from "../router/router.js";
const index = ref(0)

// 回调
const emit = defineEmits(['onChange'])
emit('onChange',index.value)
</script>

<style scoped lang="less">
.sidebar {
  width: 120px;
  height: 100%;
  background-color: bisque;
  user-select: none;
  .tab {
    display: flex;
    align-items: center;
    height: 35px;
    .tab-text{
      padding-left: 18px;
    }
  }
  .selectedTab{
    background-color: #ffbda4;
    cursor: pointer;
  }
  .unselectedTab:hover {
    cursor: pointer;
    background-color: aquamarine;
  }
}
</style>