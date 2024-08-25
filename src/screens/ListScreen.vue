<template>
  <v-container>
    <v-row>
      <v-col>
        <h3>我的列表</h3>
        <v-card calss="mx-auto">
          <v-list>
            <ListCard
              v-for="(data, index) in listData"
              :title="data.title"
              :icon-color="data.iconColor || 'blue'"
              :taskRemain="data.taskRemain"
              :key="index"
              :value="index"
            />
          </v-list>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import ListCard from '../components/ListCard.vue'
import { ListCardData } from '../utils/typing'

const listData: ListCardData[] = [
  {
    title: '提醒',
    taskRemain: 0,
    iconColor: 'red',
  },
  {
    title: '我的计划',
    taskRemain: 3,
  },
]
</script>

<style scoped></style>
