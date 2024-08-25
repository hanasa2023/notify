<template>
  <v-app>
    <v-app-bar title="Notify" color="blue">
      <template #prepend>
        <v-app-bar-nav-icon></v-app-bar-nav-icon>
      </template>
    </v-app-bar>

    <v-main>
      <v-window v-model="onBoarding">
        <v-window-item :value="0">
          <HomeScreen />
        </v-window-item>
        <v-window-item :value="1">
          <ListScreen />
        </v-window-item>
      </v-window>
    </v-main>

    <v-fab icon="mdi-plus" color="blue-darken-1" class="add-list-btn"></v-fab>
    <v-bottom-navigation bg-color="blue-lighten-1">
      <v-btn value="list" @click="onBoarding = 0">
        <v-icon>mdi-home</v-icon>
        <span>主页</span>
      </v-btn>
      <v-btn value="list" @click="onBoarding = 1">
        <v-icon>mdi-view-list</v-icon>
        <span>列表</span>
      </v-btn>
    </v-bottom-navigation>
  </v-app>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import HomeScreen from './screens/HomeScreen.vue'
import ListScreen from './screens/ListScreen.vue'

const onBoarding = ref<number>(0)
</script>

<style scoped>
.add-list-btn {
  position: fixed;
  bottom: 88px;
  right: 60px;
}
</style>
