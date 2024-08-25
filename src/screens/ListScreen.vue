<template>
  <v-app-bar title="Notify" color="blue">
    <template #prepend>
      <v-btn icon="mdi-chevron-left" @click="router.back()"></v-btn>
    </template>
  </v-app-bar>
  <v-container>
    <v-row>
      <v-col>
        <h1>{{ listName }}</h1>
        <ListItem
          v-for="(data, index) in listItems"
          :key="index"
          :is-finish="data.isFinish"
          :title="data.title"
          :list-name="data.listName"
          :tag="data.tag"
          :is-flag="data.isFlag"
        />
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import ListItem from '../components/ListItem.vue'
import { ListItemData } from '../utils/typing'

const router = useRouter()

defineProps<{
  listName: string
}>()

const listItems: ListItemData[] = [
  {
    isFinish: false,
    title: 'kmp',
    listName: '暑假计划',
    tag: '#cs',
    isFlag: true,
  },
  {
    isFinish: false,
    title: 'kmp',
    listName: '暑假计划',
    isFlag: true,
  },
  {
    isFinish: true,
    title: 'kmp',
    listName: '暑假计划',
    tag: '#cs',
    isFlag: false,
  },
]
</script>

<style scoped></style>
