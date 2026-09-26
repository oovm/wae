/** Node-API native addon surface (`wae-napi` cdylib). Types only. */

export type OpenDesktopOptions = {
    url: string;
    title?: string;
    undecorated?: boolean;
};

export type ProductUpdateOptions = {
    repo: string;
    productName: string;
    nativePath: string;
    currentVersion: string;
    tag?: string;
    allowPrerelease?: boolean;
};

export type ProductUpdateStatus = {
    upToDate: boolean;
    current: string;
    latest?: string;
    tag?: string;
    releaseUrl?: string;
    assetName?: string;
};

export type WaeNativeAddon = {
    hostVersion(): string;
    handleClientMessage(json: string): string | null;
    openDesktop(options: OpenDesktopOptions): void;
    checkProductUpdate(options: ProductUpdateOptions): ProductUpdateStatus;
    applyProductUpdate(options: ProductUpdateOptions): ProductUpdateStatus;
};
