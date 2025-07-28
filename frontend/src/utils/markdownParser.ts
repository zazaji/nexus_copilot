// frontend/src/utils/markdownParser.ts
import { marked } from 'marked';
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css';

marked.setOptions({
  renderer: new marked.Renderer(),
  highlight: (code, lang) => {
    const language = hljs.getLanguage(lang) ? lang : 'plaintext';
    return hljs.highlight(code, { language }).value;
  },
  langPrefix: 'hljs language-',
  pedantic: false,
  gfm: true,
  breaks: false,
  sanitize: false,
  smartypants: false,
  xhtml: false,
});

const preprocessSearchResults = (content: string): string => {
  // Regex to find "Title: ..." followed by "URL: ..."
  const searchResultRegex = /Title: (.*?)\nURL: (.*?)(?=\n\n|Title:|$)/gs;
  return content.replace(searchResultRegex, (match, title, url) => {
    // Convert to a Markdown link
    return `[${title.trim()}](${url.trim()})`;
  });
};

export const parseMarkdown = (content: string): string => {
  const preprocessedContent = preprocessSearchResults(content);
  return marked.parse(preprocessedContent);
};