import { createApp } from 'vue'
import App from './App.vue'
import { router } from './routers'

import './styles/main.scss'
import { createVuetify } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi'
import '@mdi/font/css/materialdesignicons.css'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const vuetify = createVuetify({
  icons: {
    defaultSet: 'mdi',
    aliases: aliases,
    sets: {
      mdi,
    },
  },
  components: components,
  directives: directives,
})

createApp(App).use(vuetify).use(router).mount('#app')
