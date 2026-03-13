import fs from 'fs';
import path from 'path';

const filePath = path.join('backends', 'wae-https', 'src', 'lib.rs');
let content = fs.readFileSync(filePath, 'utf8');

const searchStr = `impl<S> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// 合并另一个路由`;

const addStr = `impl<S> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// 添加路由
    pub fn add_route<H, T>(
        &mut self,
        method: http::Method,
        path: &str,
        handler: H,
    ) where
        H: Handler<T, S>,
        T: 'static,
    {
        self.add_route_inner(method, path.to_string(), Box::new(handler));
    }

    /// 合并另一个路由`;

if (content.includes(searchStr)) {
    content = content.replace(searchStr, addStr);
    fs.writeFileSync(filePath, content, 'utf8');
    console.log('Successfully added add_route method!');
} else {
    console.log('Search string not found!');
    process.exit(1);
}
