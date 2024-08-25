<template>
  <v-list-item>
    <template #prepend>
      <v-icon :color="iconColor">mdi-format-list-bulleted</v-icon>
    </template>
    <template #title>
      <h4>{{ title }}</h4>
    </template>
    <template #append>
      <h3>{{ taskRemain }}</h3>
      <v-icon>mdi-chevron-right</v-icon>
    </template>
  </v-list-item>
</template>

<script setup lang="ts">
import { ListCardData } from '../utils/typing'

defineProps<ListCardData>()
</script>

<style scoped></style>
