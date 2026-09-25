/** storage 抽象占位。 */
export type KeyValueStore = {
    get(key: string): Promise<string | null>;
    put(key: string, value: string): Promise<void>;
};
