<template>
  <div class="overview-content">
    <h2>{{ $t('package-overview') }}</h2>
    <div class="markdown-content" v-html="renderedContent"></div>
  </div>
</template>

<script setup lang="ts">
import {useFluent} from 'fluent-vue'
import {onMounted, ref, watch} from 'vue'
import type {PackageDetail} from '@/api/models'
import MarkdownIt from 'markdown-it'
import markdownItKatex from 'markdown-it-katex'
import {createHighlighter} from 'shiki'
import 'katex/dist/katex.min.css'

const props = defineProps<{
  packageInfo: PackageDetail
}>()

const {$t} = useFluent()
const renderedContent = ref('')

const highlighter = await createHighlighter({
  themes: ['github-light'],
  langs: ['typescript'],
})

const initMarkdown = async () => {


  const md = new MarkdownIt({
    html: true,
    highlight: (code, lang) => {
      return highlighter.codeToHtml(code, {
        theme: 'github-light',
        lang: lang,
      })
    }
  })

  md.use(markdownItKatex, {
    throwOnError: false,
    errorColor: '#cc0000'
  })

  return md
}

const renderContent = async () => {
  const md = await initMarkdown()
  renderedContent.value = md.render(props.packageInfo.documentation || '')
}

onMounted(() => {
  renderContent()
})

watch(() => props.packageInfo.documentation, () => {
  renderContent()
})
</script>

<style lang="scss">
.overview-content {
  padding: 1rem;

  h2 {
    margin-bottom: 1rem;
    font-size: 1.5rem;
    color: #333;
  }

  .markdown-content {
    color: #666;
    line-height: 1.6;

    :deep(h1), :deep(h2), :deep(h3), :deep(h4), :deep(h5), :deep(h6) {
      margin: 1.5rem 0 1rem;
      color: #333;
    }

    :deep(p) {
      margin: 1rem 0;
    }

    :deep(code) {
      background-color: #f5f7fa;
      padding: 0.2rem 0.4rem;
      border-radius: 4px;
      font-family: monospace;
    }

    :deep(pre) {
      margin: 1rem 0;
      padding: 1rem;
      border-radius: 6px;
      overflow-x: auto;

      code {
        background-color: transparent;
        padding: 0;
      }
    }

    :deep(.katex-display) {
      margin: 1rem 0;
      overflow-x: auto;
      overflow-y: hidden;
    }

    :deep(a) {
      color: #4f46e5;
      text-decoration: none;

      &:hover {
        text-decoration: underline;
      }
    }

    :deep(ul), :deep(ol) {
      margin: 1rem 0;
      padding-left: 1.5rem;
    }

    :deep(blockquote) {
      margin: 1rem 0;
      padding: 0.5rem 1rem;
      border-left: 4px solid #e5e7eb;
      background-color: #f9fafb;
    }
  }
}
</style>