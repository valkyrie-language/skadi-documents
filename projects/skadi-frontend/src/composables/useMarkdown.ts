import MarkdownIt from 'markdown-it'
import markdownItKatex from 'markdown-it-katex'
import {createHighlighter} from 'shiki'
import 'katex/dist/katex.min.css'

export const useMarkdown = async () => {
  const highlighter = await createHighlighter({
    themes: ['github-light'],
    langs: ['typescript'],
  })

  const initMarkdown = async () => {
    const md = new MarkdownIt({
      html: true,
      highlight: (code: string, lang: string) => {
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

  const renderMarkdown = async (content: string) => {
    const md = await initMarkdown()
    return md.render(content || '')
  }

  return {
    renderMarkdown
  }
}