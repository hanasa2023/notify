<template>
  <v-card @click="">
    <template #prepend>
      <v-icon>{{ icon }}</v-icon>
    </template>
    <template #append>
      <h2>{{ taskRemain }}</h2>
    </template>
    <template #actions>
      <h4>{{ title }}</h4>
    </template>
  </v-card>
</template>

<script setup lang="ts">
import { DefaultCardProps } from '../utils/typing'

defineProps<DefaultCardProps>()
</script>

<style scoped></style>
