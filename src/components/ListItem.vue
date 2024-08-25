<template>
  <v-list-item @click="">
    <template #prepend>
      <v-icon>{{ icon }}</v-icon>
    </template>
    <template #append v-if="isFlag || false">
      <v-icon>mdi-flag</v-icon>
    </template>
    <template #title>
      <h4>{{ title }}</h4>
    </template>
    <template #subtitle v-if="tag"> {{ tag }} </template>
  </v-list-item>
  <v-divider inset></v-divider>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ListItemData } from '../utils/typing'
const props = defineProps<ListItemData>()

const icon = computed<string>(() => {
  return props.isFinish ? 'mdi-circle-slice-8' : 'mdi-circle-outline'
})
</script>

<style scoped></style>
