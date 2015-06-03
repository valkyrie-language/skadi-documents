import { createFluentVue } from 'fluent-vue'
import { createFluentBundle } from '@fluent/bundle'

const bundles = {
  'zh': createFluentBundle('zh', {
    functions: {
      VERSION: () => '1.0.0'
    }
  }),
  'en': createFluentBundle('en', {
    functions: {
      VERSION: () => '1.0.0'
    }
  })
}

export const i18n = createFluentVue({
  bundles
})