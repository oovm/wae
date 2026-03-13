#!/usr/bin/env node

import fs from 'fs';
import path from 'path';

const projectDir = 'e:\\灵之镜有限公司\\wae';
const libPath = path.join(projectDir, 'backends', 'wae-https', 'src', 'lib.rs');
const tempPath = path.join(projectDir, 'backends', 'wae-https', 'src', 'lib.rs.temp');

console.log('读取 temp 文件...');
const tempContent = fs.readFileSync(tempPath, 'utf8');

console.log('修改内容，添加必要的功能...');

// 修改 Router 结构体，支持处理函数的存储和调用
// 修改 handle_request 方法，实际分发请求

console.log('写入 lib.rs...');
fs.writeFileSync(libPath, tempContent, 'utf8');

console.log('完成！');
