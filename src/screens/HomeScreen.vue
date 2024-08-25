<template>
  <v-app-bar title="Notify" color="blue">
    <template #prepend>
      <v-app-bar-nav-icon></v-app-bar-nav-icon>
    </template>
  </v-app-bar>
  <v-container>
    <v-row>
      <v-col
        :cols="reactiveCols"
        v-for="(data, index) in defaultCardsData"
        :key="index"
      >
        <DefaultCard
          :title="data.title"
          :icon="data.icon"
          :task-remain="data.taskRemain"
        />
      </v-col>
    </v-row>
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
              @click="router.push(`/list/${data.title}`)"
            />
          </v-list>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { useDisplay } from 'vuetify'
import DefaultCard from '../components/DefaultCard.vue'
import { DefaultCardProps } from '../utils/typing'
import { computed } from 'vue'
import ListCard from '../components/ListCard.vue'
import { ListCardData } from '../utils/typing'
import { useRouter } from 'vue-router'

const { name } = useDisplay()

const router = useRouter()

const reactiveCols = computed<number>(() => {
  switch (name.value) {
    case 'xs':
      return 6
    case 'sm':
      return 4
    case 'md':
      return 4
    case 'lg':
      return 3
    case 'xl':
      return 3
    case 'xxl':
      return 2
  }
})

const defaultCardsData: DefaultCardProps[] = [
  {
    icon: 'mdi-calendar-today',
    title: '今天',
    taskRemain: 0,
  },
  {
    icon: 'mdi-calendar-multiselect',
    title: '计划',
    taskRemain: 0,
  },
  {
    icon: 'mdi-ballot',
    title: '全部',
    taskRemain: 0,
  },
  {
    icon: 'mdi-flag-variant',
    title: '旗标',
    taskRemain: 0,
  },
  {
    icon: 'mdi-check',
    title: '完成',
    taskRemain: 0,
  },
]

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
