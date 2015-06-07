import {FluentBundle, FluentResource} from '@fluent/bundle'

import {createFluentVue} from 'fluent-vue'

import en_us from "./en-us.ftl?raw"
import zh_hans from "./zh-hans.ftl?raw"



const enBundle = (new FluentBundle('en-US'))
enBundle.addResource(new FluentResource(en_us))

const zhBundle = new FluentBundle('zh-Hans')
zhBundle.addResource(new FluentResource(zh_hans))

export const fluent = createFluentVue({
    bundles: [enBundle, zhBundle]
})

export default fluent;