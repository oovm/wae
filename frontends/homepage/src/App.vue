<template>
  <div class="min-h-screen">
    <header class="bg-white/90 backdrop-blur-xl shadow-lg border-b border-slate-200 sticky top-0 z-50 transition-all duration-300">
      <div class="container mx-auto px-4">
        <div class="flex items-center justify-between h-20">
          <div class="flex items-center gap-3 cursor-pointer" @click="currentSection = 'home'">
            <div class="w-12 h-12 bg-gradient-to-br from-orange-500 via-red-600 to-orange-700 rounded-xl flex items-center justify-center shadow-lg hover:shadow-xl transition-all duration-300 transform hover:scale-105">
              <span class="text-white font-extrabold text-2xl">W</span>
            </div>
            <span class="text-2xl font-extrabold bg-gradient-to-r from-orange-600 via-red-600 to-orange-700 bg-clip-text text-transparent hover:scale-[1.02] transition-transform">WAE</span>
          </div>
          
          <nav class="flex items-center gap-2">
            <button 
              @click="currentSection = 'home'"
              :class="[
                'px-5 py-3 rounded-xl font-semibold transition-all duration-300 flex items-center gap-2',
                currentSection === 'home' 
                  ? 'text-white bg-gradient-to-r from-blue-600 to-purple-700 shadow-lg' 
                  : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'
              ]"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001-1m-6 0h6"></path>
              </svg>
              Home
            </button>
            <button 
              @click="currentSection = 'docs'"
              :class="[
                'px-5 py-3 rounded-xl font-semibold transition-all duration-300 flex items-center gap-2',
                currentSection === 'docs' 
                  ? 'text-white bg-gradient-to-r from-blue-600 to-purple-700 shadow-lg' 
                  : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'
              ]"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
              </svg>
              Documentation
            </button>
          </nav>
        </div>
      </div>
    </header>

    <div v-if="currentSection === 'home'">
      <section class="bg-gradient-to-br from-orange-600 via-red-700 to-orange-800 relative overflow-hidden">
        <div class="absolute inset-0 overflow-hidden">
          <div class="absolute -top-40 -left-40 w-96 h-96 bg-orange-400 rounded-full mix-blend-multiply filter blur-3xl opacity-30"></div>
          <div class="absolute -bottom-40 -right-40 w-96 h-96 bg-red-400 rounded-full mix-blend-multiply filter blur-3xl opacity-30"></div>
        </div>
        
        <div class="container mx-auto px-4 py-24 relative z-10">
          <div class="grid lg:grid-cols-2 gap-12 items-center">
            <div class="text-white">
              <h1 class="text-5xl md:text-6xl font-extrabold mb-6 leading-tight">
                WAE - Rust Async Utilities
              </h1>
              <p class="text-xl md:text-2xl text-orange-100 mb-8 leading-relaxed">
                微服务优先的 Rust 异步框架，完全替代 axum，深度融合 tokio，提供一站式全栈解决方案
              </p>
              <div class="flex flex-wrap gap-4">
                <button 
                  @click="currentSection = 'docs'"
                  class="px-8 py-4 bg-white text-orange-600 rounded-xl font-bold text-lg hover:bg-orange-50 transition-all shadow-xl hover:shadow-2xl transform hover:-translate-y-0.5"
                >
                  查看文档
                </button>
                <button 
                  @click="scrollToFeatures"
                  class="px-8 py-4 border-2 border-white text-white rounded-xl font-bold text-lg hover:bg-white/10 transition-all"
                >
                  了解更多
                </button>
              </div>
            </div>
            
            <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-4 shadow-2xl">
              <div class="h-96 bg-slate-900 rounded-xl p-4 overflow-auto">
                <pre class="text-sm text-green-400 font-mono"><code>use axum::{Router, routing::{get, post}, Json};
use wae_https::{HttpsServerBuilder, ApiResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

async fn hello() -> ApiResponse&lt;HelloResponse&gt; {
    ApiResponse::success(HelloResponse {
        message: "Hello, WAE!".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result&lt;(), Box&lt;dyn std::error::Error&gt;&gt; {
    let router = Router::new()
        .route("/hello", get(hello));

    let server = HttpsServerBuilder::new()
        .addr("0.0.0.0:3000".parse()?)
        .service_name("my-service")
        .router(router)
        .build();

    server.serve().await?;
    Ok(())
}</code></pre>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section id="features" class="py-24 bg-white">
        <div class="container mx-auto px-4">
          <div class="text-center mb-16">
            <h2 class="text-4xl md:text-5xl font-extrabold bg-gradient-to-r from-orange-600 to-red-700 bg-clip-text text-transparent mb-4">
              核心特性
            </h2>
            <p class="text-xl text-gray-600 max-w-2xl mx-auto">
              专为现代微服务设计的高性能全栈解决方案
            </p>
          </div>
          
          <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
            <div class="group relative bg-gradient-to-br from-slate-50 to-slate-100 rounded-2xl p-8 border border-slate-200 hover:border-orange-300 transition-all duration-300 hover:shadow-2xl hover:-translate-y-2 overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-br from-orange-500/5 to-red-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
              <div class="relative z-10">
                <div class="w-20 h-20 bg-gradient-to-br from-orange-500 to-red-600 rounded-2xl flex items-center justify-center mb-6 shadow-lg group-hover:shadow-orange-500/30 transition-all duration-300">
                  <span class="text-4xl">🚀</span>
                </div>
                <h3 class="text-2xl font-bold text-gray-900 mb-3">微服务优先</h3>
                <p class="text-gray-600 leading-relaxed">
                  服务发现、配置中心、链路追踪、健康检查，开箱即用
                </p>
              </div>
            </div>
            
            <div class="group relative bg-gradient-to-br from-slate-50 to-slate-100 rounded-2xl p-8 border border-slate-200 hover:border-orange-300 transition-all duration-300 hover:shadow-2xl hover:-translate-y-2 overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-br from-orange-500/5 to-red-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
              <div class="relative z-10">
                <div class="w-20 h-20 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-2xl flex items-center justify-center mb-6 shadow-lg group-hover:shadow-blue-500/30 transition-all duration-300">
                  <span class="text-4xl">⚡</span>
                </div>
                <h3 class="text-2xl font-bold text-gray-900 mb-3">深度融合 tokio</h3>
                <p class="text-gray-600 leading-relaxed">
                  基于 axum 构建，原生 async/await，极致性能
                </p>
              </div>
            </div>
            
            <div class="group relative bg-gradient-to-br from-slate-50 to-slate-100 rounded-2xl p-8 border border-slate-200 hover:border-orange-300 transition-all duration-300 hover:shadow-2xl hover:-translate-y-2 overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-br from-orange-500/5 to-red-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
              <div class="relative z-10">
                <div class="w-20 h-20 bg-gradient-to-br from-green-500 to-teal-600 rounded-2xl flex items-center justify-center mb-6 shadow-lg group-hover:shadow-green-500/30 transition-all duration-300">
                  <span class="text-4xl">🤖</span>
                </div>
                <h3 class="text-2xl font-bold text-gray-900 mb-3">AI 友好</h3>
                <p class="text-gray-600 leading-relaxed">
                  清晰的 Trait 抽象，统一的错误处理，最小化样板代码
                </p>
              </div>
            </div>
            
            <div class="group relative bg-gradient-to-br from-slate-50 to-slate-100 rounded-2xl p-8 border border-slate-200 hover:border-orange-300 transition-all duration-300 hover:shadow-2xl hover:-translate-y-2 overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-br from-orange-500/5 to-red-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
              <div class="relative z-10">
                <div class="w-20 h-20 bg-gradient-to-br from-purple-500 to-pink-600 rounded-2xl flex items-center justify-center mb-6 shadow-lg group-hover:shadow-purple-500/30 transition-all duration-300">
                  <span class="text-4xl">🦀</span>
                </div>
                <h3 class="text-2xl font-bold text-gray-900 mb-3">纯血 Rust</h3>
                <p class="text-gray-600 leading-relaxed">
                  零 FFI 绑定，跨平台支持，内存安全，可审计
                </p>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section class="py-24 bg-gradient-to-br from-slate-50 to-slate-100">
        <div class="container mx-auto px-4">
          <div class="text-center mb-16">
            <h2 class="text-4xl md:text-5xl font-extrabold bg-gradient-to-r from-orange-600 to-red-700 bg-clip-text text-transparent mb-4">
              快速开始
            </h2>
            <p class="text-xl text-gray-600 max-w-2xl mx-auto">
              几分钟内即可启动你的 WAE 服务
            </p>
          </div>

          <div class="grid md:grid-cols-3 gap-8 max-w-5xl mx-auto mb-12">
            <div class="bg-white rounded-2xl shadow-xl border border-slate-200 p-8">
              <div class="w-14 h-14 bg-orange-100 text-orange-600 rounded-xl flex items-center justify-center text-2xl font-bold mb-6">1</div>
              <h3 class="text-xl font-bold text-gray-900 mb-4">添加依赖</h3>
              <div class="bg-slate-900 rounded-xl p-4 mb-4">
                <code class="text-green-400 font-mono text-sm">wae = { path = "wae" }</code>
              </div>
              <p class="text-gray-600 text-sm">在 Cargo.toml 中添加 WAE 依赖</p>
            </div>

            <div class="bg-white rounded-2xl shadow-xl border border-slate-200 p-8">
              <div class="w-14 h-14 bg-orange-100 text-orange-600 rounded-xl flex items-center justify-center text-2xl font-bold mb-6">2</div>
              <h3 class="text-xl font-bold text-gray-900 mb-4">编写路由</h3>
              <div class="bg-slate-900 rounded-xl p-4 mb-4">
                <code class="text-green-400 font-mono text-sm">Router::new()</code>
              </div>
              <p class="text-gray-600 text-sm">使用 axum Router 构建你的 API</p>
            </div>

            <div class="bg-white rounded-2xl shadow-xl border border-slate-200 p-8">
              <div class="w-14 h-14 bg-orange-100 text-orange-600 rounded-xl flex items-center justify-center text-2xl font-bold mb-6">3</div>
              <h3 class="text-xl font-bold text-gray-900 mb-4">启动服务</h3>
              <div class="bg-slate-900 rounded-xl p-4 mb-4">
                <code class="text-green-400 font-mono text-sm">cargo run</code>
              </div>
              <p class="text-gray-600 text-sm">一行命令启动你的 WAE 服务</p>
            </div>
          </div>

          <div class="max-w-4xl mx-auto">
            <div class="bg-white rounded-2xl shadow-xl border border-slate-200 overflow-hidden">
              <div class="flex items-center justify-between px-6 py-4 bg-gradient-to-r from-orange-600 to-red-700">
                <h3 class="text-xl font-bold text-white flex items-center gap-2">
                  <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path>
                  </svg>
                  Cargo.toml
                </h3>
              </div>
              <div class="p-6 bg-slate-900">
                <pre class="text-sm text-slate-300 font-mono leading-relaxed"><code>[dependencies]
wae = { path = "wae" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }</code></pre>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section class="py-24 bg-white">
        <div class="container mx-auto px-4">
          <div class="text-center mb-16">
            <h2 class="text-4xl md:text-5xl font-extrabold bg-gradient-to-r from-orange-600 to-red-700 bg-clip-text text-transparent mb-4">
              模块概览
            </h2>
            <p class="text-xl text-gray-600 max-w-2xl mx-auto">
              模块化的 Crate 设计，每个模块可独立使用
            </p>
          </div>
          
          <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto">
            <div class="bg-gradient-to-br from-slate-50 to-slate-100 rounded-xl p-6 border border-slate-200">
              <h3 class="text-xl font-bold text-gray-900 mb-3">wae-types</h3>
              <p class="text-gray-600">核心类型定义，统一错误类型与结果类型</p>
            </div>
            <div class="bg-gradient-to-br from-slate-50 to-slate-100 rounded-xl p-6 border border-slate-200">
              <h3 class="text-xl font-bold text-gray-900 mb-3">wae-https</h3>
              <p class="text-gray-600">HTTP/HTTPS 服务，基于 axum 构建，统一响应结构</p>
            </div>
            <div class="bg-gradient-to-br from-slate-50 to-slate-100 rounded-xl p-6 border border-slate-200">
              <h3 class="text-xl font-bold text-gray-900 mb-3">wae-authentication</h3>
              <p class="text-gray-600">认证服务 (JWT, OAuth2, SAML, TOTP)</p>
            </div>
            <div class="bg-gradient-to-br from-slate-50 to-slate-100 rounded-xl p-6 border border-slate-200">
              <h3 class="text-xl font-bold text-gray-900 mb-3">wae-ai</h3>
              <p class="text-gray-600">AI 服务抽象，支持腾讯混元、火山引擎等</p>
            </div>
            <div class="bg-gradient-to-br from-slate-50 to-slate-100 rounded-xl p-6 border border-slate-200">
              <h3 class="text-xl font-bold text-gray-900 mb-3">wae-storage</h3>
              <p class="text-gray-600">存储服务抽象，支持腾讯云 COS、阿里云 OSS、本地存储</p>
            </div>
            <div class="bg-gradient-to-br from-slate-50 to-slate-100 rounded-xl p-6 border border-slate-200">
              <h3 class="text-xl font-bold text-gray-900 mb-3">wae-config</h3>
              <p class="text-gray-600">多层级配置管理，支持 TOML/YAML/环境变量</p>
            </div>
          </div>
        </div>
      </section>
    </div>

    <div v-if="currentSection === 'docs'" class="container mx-auto px-4 py-8">
      <div class="flex gap-8">
        <aside class="w-80 flex-shrink-0">
          <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-6 sticky top-24">
            <h2 class="text-xl font-bold text-gray-800 mb-4">文档导航</h2>
            <nav v-if="docTree.length > 0">
              <div v-for="node in docTree" :key="node.id" class="mb-2">
                <DocTreeNode :node="node" :current-path="currentDocPath" @select="selectDoc" />
              </div>
            </nav>
            <div v-else class="text-gray-500 text-sm">
              加载文档中...
            </div>
          </div>
        </aside>

        <main class="flex-1">
          <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-8">
            <div v-if="currentDoc">
              <h2 class="text-3xl font-bold mb-6 text-gray-800">{{ currentDoc.title }}</h2>
              <MarkdownViewer :content="currentDocContent" />
            </div>
            <div v-else class="text-gray-500 text-center py-12">
              <p class="text-lg">请从左侧选择文档查看</p>
            </div>
          </div>
        </main>
      </div>
    </div>

    <footer class="bg-slate-900 text-slate-300">
      <div class="container mx-auto px-4 py-16">
        <div class="grid md:grid-cols-4 gap-12">
          <div class="md:col-span-2">
            <div class="flex items-center gap-3 mb-6">
              <div class="w-12 h-12 bg-gradient-to-br from-orange-500 via-red-600 to-orange-700 rounded-xl flex items-center justify-center">
                <span class="text-white font-extrabold text-2xl">W</span>
              </div>
              <span class="text-2xl font-extrabold text-white">WAE</span>
            </div>
            <p class="text-slate-400 mb-6 leading-relaxed max-w-md">
              微服务优先的 Rust 异步框架，完全替代 axum，深度融合 tokio，提供一站式全栈解决方案。
            </p>
            <div class="flex gap-4">
              <a href="https://github.com/oovm/wae" class="w-10 h-10 bg-slate-800 hover:bg-slate-700 rounded-lg flex items-center justify-center transition-colors">
                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
              </a>
            </div>
          </div>
          
          <div>
            <h4 class="text-white font-semibold text-lg mb-6">文档资源</h4>
            <ul class="space-y-3">
              <li><a href="#" class="text-slate-400 hover:text-white transition-colors">使用文档</a></li>
              <li><a href="#" class="text-slate-400 hover:text-white transition-colors">API 参考</a></li>
              <li><a href="#" class="text-slate-400 hover:text-white transition-colors">示例项目</a></li>
              <li><a href="#" class="text-slate-400 hover:text-white transition-colors">教程指南</a></li>
            </ul>
          </div>
          
          <div>
            <h4 class="text-white font-semibold text-lg mb-6">项目信息</h4>
            <ul class="space-y-3">
              <li><a href="https://github.com/oovm/wae" class="text-slate-400 hover:text-white transition-colors">GitHub</a></li>
              <li><a href="#" class="text-slate-400 hover:text-white transition-colors">更新日志</a></li>
              <li><a href="#" class="text-slate-400 hover:text-white transition-colors">贡献指南</a></li>
              <li><a href="#" class="text-slate-400 hover:text-white transition-colors">问题反馈</a></li>
            </ul>
          </div>
        </div>
        
        <div class="border-t border-slate-800 mt-12 pt-8 flex flex-col md:flex-row justify-between items-center gap-4">
          <p class="text-slate-500 text-sm">
            © 2024 WAE. 保留所有权利。
          </p>
          <div class="flex gap-6 text-sm">
            <a href="#" class="text-slate-500 hover:text-white transition-colors">隐私政策</a>
            <a href="#" class="text-slate-500 hover:text-white transition-colors">服务条款</a>
            <a href="#" class="text-slate-500 hover:text-white transition-colors">行为准则</a>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { loadDocs, getDocContent, type DocNode } from '@/docs'
import MarkdownViewer from '@/components/MarkdownViewer.vue'
import DocTreeNode from '@/components/DocTreeNode.vue'

const currentSection = ref<'home' | 'docs'>('home')
const docTree = ref<DocNode[]>([])
const currentDocPath = ref<string>('')
const currentDoc = ref<DocNode | null>(null)
const currentDocContent = ref<string>('')

async function init() {
  console.log('Initializing docs...')
  docTree.value = await loadDocs()
  console.log('Loaded docTree:', docTree.value)
  if (docTree.value.length > 0) {
    const firstDoc = findFirstDoc(docTree.value[0])
    if (firstDoc) {
      selectDoc(firstDoc)
    }
  }
}

function findFirstDoc(node: DocNode): DocNode | null {
  if (!node.isDirectory && !node.children?.length) {
    return node
  }
  if (node.children && node.children.length > 0) {
    return findFirstDoc(node.children[0])
  }
  return null
}

async function selectDoc(node: DocNode) {
  currentDocPath.value = node.path
  currentDoc.value = node
  currentDocContent.value = await getDocContent(node.path)
}

function scrollToFeatures() {
  const featuresSection = document.getElementById('features')
  featuresSection?.scrollIntoView({ behavior: 'smooth' })
}

onMounted(() => {
  init()
})
</script>
