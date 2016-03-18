/** Queues consumer 占位。属于 Cloudflare adapter 能力，不进 @wae/serverless。 */

export type QueueMessage<T = unknown> = {
    id: string;
    body: T;
    ack(): void;
    retry(): void;
};

export type QueueBatch<T = unknown> = {
    messages: QueueMessage<T>[];
};

export type QueueHandler<T = unknown, Env = unknown> = (batch: QueueBatch<T>, env: Env) => Promise<void> | void;
