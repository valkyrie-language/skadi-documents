import { createFluentVue } from 'fluent-vue'
import { createFluentBundle } from '@fluent/bundle'

const zhResource = await fetch('/src/locales/zh.ftl').then(r => r.text())
const enResource = await fetch('/src/locales/en.ftl').then(r => r.text())

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

bundles.zh.addResource(zhResource)
bundles.en.addResource(enResource)

export const i18n = createFluentVue({
  bundles
})