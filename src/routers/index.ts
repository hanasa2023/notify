import { createMemoryHistory, createRouter } from 'vue-router'
import HomeScreen from '../screens/HomeScreen.vue'
import ListScreen from '../screens/ListScreen.vue'

declare module 'vue-router' {
  interface RouteMeta {
    transition: string
  }
}

const routes = [
  { path: '/', component: HomeScreen, meta: { transition: 'slide-right' } },
  {
    path: '/list/:listName',
    component: ListScreen,
    props: true,
    meta: { transition: 'slide-left' },
  },
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})
