<template>
  <v-app>
    <v-main>
      <router-view> </router-view>
    </v-main>

    <v-fab icon="mdi-plus" color="blue-darken-1" class="add-list-btn"></v-fab>
  </v-app>
</template>

<script setup lang="ts"></script>

<style scoped>
.add-list-btn {
  position: fixed;
  bottom: 40px;
  right: 60px;
}
</style>
