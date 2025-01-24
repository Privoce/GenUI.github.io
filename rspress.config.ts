import * as path from "node:path";
import { defineConfig } from "rspress/config";
import alignImage from "rspress-plugin-align-image";

export default defineConfig({
  base:"/GenUI.github.io/",
  root: path.join(__dirname, "docs"),
  title: "GenUI Book",
  description: "A book for GenUI",
  icon: "/genui.png",
  logo: {
    light: "/genui.png",
    dark: "/genui.png",
  },
  lang: "en",
  globalStyles: path.join(__dirname, 'theme', 'index.css'),
  plugins: [alignImage({
    justify: 'center',
    containerClassNames: ['img-center'],
  })],
  themeConfig: {
    enableContentAnimation: true,
    locales: [
      {
        lang: "en",
        label: "English",
        title: "GenUI Book",
        description: "A book for GenUI",
        outlineTitle: 'Table of Contents',
      },
      {
        lang: "zh",
        label: "中文",
        title: "GenUI Book",
        description: "GenUI 的教程",
        outlineTitle: '目录',
      },
    ],
    socialLinks: [
      {
        icon: "github",
        mode: "link",
        content: "https://github.com/Privoce/GenUI",
      },
    ],
  },
  markdown: {
    showLineNumbers: true,
    checkDeadLinks: true,
    highlightLanguages:[['rs', 'rust']]
  },
  route: {
    cleanUrls: true,
  },
  multiVersion: {
    default: "v0.1.0",
    versions: ["v0.1.0"],
  },
  search: {
    versioned: true,
  },
  
});
