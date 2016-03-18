/** Durable Objects — 有状态对象生命周期，不是普通 serverless handler。 */

export abstract class WaeDurableObject {
    abstract fetch(request: Request): Promise<Response>;

    protected handle(request: Request): Promise<Response> {
        return this.fetch(request);
    }
}
