export interface DocNode {
  id: string
  title: string
  path: string
  isDirectory: boolean
  children?: DocNode[]
  parentId?: string
  order?: number
}

export interface DocMetadata {
  title?: string
  order?: number
}

import { docsModules as rawDocsModules } from 'virtual:docs'

const docsModules: Record<string, string> = {}
for (const key in rawDocsModules) {
  const normalizedKey = key.replace(/\\/g, '/')
  docsModules[`../../documentation/zh-hans/${normalizedKey}`] = rawDocsModules[key]
}

console.log('Loaded docsModules:', Object.keys(docsModules))

function parseDocMetadata(content: string): DocMetadata {
  const metadata: DocMetadata = {}
  const frontmatterMatch = content.match(/^---\n([\s\S]*?)\n---/)
  
  if (frontmatterMatch) {
    const frontmatter = frontmatterMatch[1]
    const titleMatch = frontmatter.match(/^title:\s*(.+)$/m)
    const orderMatch = frontmatter.match(/^order:\s*(\d+)$/m)
    
    if (titleMatch) {
      metadata.title = titleMatch[1].trim().replace(/^['"]|['"]$/g, '')
    }
    if (orderMatch) {
      metadata.order = parseInt(orderMatch[1], 10)
    }
  }
  
  return metadata
}

function normalizePath(path: string): string {
  return path.replace(/\\/g, '/')
}

function generateId(path: string): string {
  const normalizedPath = normalizePath(path)
  return normalizedPath
    .replace('../../documentation/zh-hans/', '')
    .replace(/\.md$/, '')
    .replace(/\//g, '-')
}

function getDocTitle(path: string, metadata: DocMetadata): string {
  if (metadata.title) {
    return metadata.title
  }
  
  const normalizedPath = normalizePath(path)
  const filename = normalizedPath.split('/').pop()?.replace('.md', '') || ''
  
  const titleMap: Record<string, string> = {
    'index': '首页',
    'introduction': '介绍',
    'quick-start': '快速开始',
    'features': '功能特性',
    'grammar': '语法',
    'guide': '指南',
    'rust-usage': 'Rust 使用',
    'maintenance': '维护',
    'parser': '解析器',
    'testing': '测试',
    'faq': '常见问题',
    'readme': '说明'
  }
  
  return titleMap[filename] || filename
}

function buildDocTree(docs: DocNode[]): DocNode[] {
  const nodeMap: Record<string, DocNode> = {}
  const pathToNode: Record<string, DocNode> = {}
  
  docs.forEach(doc => {
    const normalizedPath = normalizePath(doc.path)
    const node = { ...doc, path: normalizedPath, children: [] }
    nodeMap[doc.id] = node
    pathToNode[normalizedPath] = node
  })
  
  const root: DocNode[] = []
  
  docs.forEach(doc => {
    const normalizedPath = normalizePath(doc.path)
    const pathParts = normalizedPath.split('/')
    const relativeParts = pathParts.slice(4)
    
    if (relativeParts.length === 1) {
      if (relativeParts[0] !== 'readme.md') {
        root.push(pathToNode[normalizedPath])
      }
    }
  })
  
  docs.forEach(doc => {
    const normalizedPath = normalizePath(doc.path)
    const pathParts = normalizedPath.split('/')
    const relativeParts = pathParts.slice(4)
    
    if (relativeParts.length > 1) {
      const childNode = pathToNode[normalizedPath]
      
      for (let i = relativeParts.length - 1; i >= 1; i--) {
        const parentRelativeParts = relativeParts.slice(0, i)
        const parentPathParts = [...pathParts.slice(0, 4), ...parentRelativeParts, 'index.md']
        const parentPath = parentPathParts.join('/')
        const parentNode = pathToNode[parentPath]
        
        if (parentNode && parentNode.children) {
          parentNode.children.push(childNode)
          break
        }
      }
    }
  })
  
  const sortNodes = (nodes: DocNode[]): DocNode[] => {
    return nodes
      .sort((a, b) => {
        if (a.path.endsWith('/index.md') && !b.path.endsWith('/index.md')) return -1
        if (!a.path.endsWith('/index.md') && b.path.endsWith('/index.md')) return 1
        return (a.order || 999) - (b.order || 999)
      })
      .map(node => ({
        ...node,
        children: node.children ? sortNodes(node.children) : undefined
      }))
  }
  
  return sortNodes(root)
}

export async function loadDocs(): Promise<DocNode[]> {
  console.log('docsModules keys:', Object.keys(docsModules))
  const docs: DocNode[] = []
  
  for (const path in docsModules) {
    const content = docsModules[path] as string
    const metadata = parseDocMetadata(content)
    const id = generateId(path)
    const title = getDocTitle(path, metadata)
    
    docs.push({
      id,
      title,
      path,
      isDirectory: false,
      order: metadata.order
    })
  }
  
  console.log('Loaded docs:', docs)
  const tree = buildDocTree(docs)
  console.log('Built doc tree:', tree)
  return tree
}

export async function getDocContent(path: string): Promise<string> {
  const normalizedPath = normalizePath(path)
  return docsModules[normalizedPath] as string || docsModules[path] as string || ''
}
