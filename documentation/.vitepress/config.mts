import { defineConfig } from 'vitepress'

export default defineConfig({
    title: 'WAE',
    description: '微服务优先的 Rust 异步框架',
    lang: 'zh-CN',
    srcDir: 'zh-hans',
    outDir: 'dist',
    cacheDir: 'node_modules/.vitepress-cache',
    themeConfig: {
        logo: '/logo.svg',
        nav: [
            { text: '指南', link: '/guide/introduction' },
            { text: '架构', link: '/architecture/overview' },
            { text: '模块', link: '/modules/types' },
            { text: 'API', link: '/api/overview' }
        ],
        sidebar: {
            '/guide/': [
                {
                    text: '开始',
                    items: [
                        { text: '简介', link: '/guide/introduction' },
                        { text: '快速开始', link: '/guide/getting-started' },
                        { text: '核心优势', link: '/guide/advantages' }
                    ]
                }
            ],
            '/architecture/': [
                {
                    text: '架构设计',
                    items: [
                        { text: '概览', link: '/architecture/overview' },
                        { text: '分层架构', link: '/architecture/layers' },
                        { text: '设计模式', link: '/architecture/patterns' }
                    ]
                }
            ],
            '/modules/': [
                {
                    text: '核心模块',
                    items: [
                        { text: 'wae-types', link: '/modules/types' },
                        { text: 'wae-config', link: '/modules/wae-config' },
                        { text: 'wae-https', link: '/modules/http' }
                    ]
                },
                {
                    text: '服务模块',
                    items: [
                        { text: 'wae-service', link: '/modules/wae-service' },
                        { text: 'wae-storage', link: '/modules/storage' },
                        { text: 'wae-cache', link: '/modules/wae-cache' },
                        { text: 'wae-database', link: '/modules/wae-database' }
                    ]
                },
                {
                    text: '通信模块',
                    items: [
                        { text: 'wae-websocket', link: '/modules/wae-websocket' },
                        { text: 'wae-email', link: '/modules/email' }
                    ]
                },
                {
                    text: '能力模块',
                    items: [
                        { text: 'wae-ai', link: '/modules/ai' },
                        { text: 'wae-distributed', link: '/modules/wae-distributed' }
                    ]
                }
            ],
            '/api/': [
                {
                    text: 'API 参考',
                    items: [
                        { text: '概览', link: '/api/overview' }
                    ]
                }
            ]
        },
        socialLinks: [
            { icon: 'github', link: 'https://github.com/your-org/wae' }
        ],
        footer: {
            message: '基于 MIT 许可发布',
            copyright: 'Copyright © 2024 WAE Team'
        },
        search: {
            provider: 'local'
        }
    }
})
