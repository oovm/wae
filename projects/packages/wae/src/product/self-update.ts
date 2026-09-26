/** Self-update for `wae build` products — loads the shipped `.node` from the product tree. */

import { createRequire } from "node:module";
import type { ProductUpdateOptions, ProductUpdateStatus, WaeNativeAddon, WaeProductManifest } from "@wae/types";
import { WAE_PRODUCT_MANIFEST } from "@wae/types";
import { loadProductManifest, resolveNativeAbsolutePath } from "./manifest.js";

function loadNativeAddon(nativePath: string): WaeNativeAddon {
    const req = createRequire(import.meta.url);
    return req(nativePath) as WaeNativeAddon;
}

function toNativeOptions(manifest: WaeProductManifest, productRoot: string): ProductUpdateOptions {
    const update = manifest.update;
    if (!update?.github) {
        throw new Error(`${WAE_PRODUCT_MANIFEST} missing product.update.github`);
    }
    return {
        repo: update.github,
        productName: manifest.name,
        nativePath: resolveNativeAbsolutePath(manifest, productRoot),
        currentVersion: manifest.version,
        allowPrerelease: update.allowPrerelease,
    };
}

export function checkProductUpdateFromManifest(
    manifest: WaeProductManifest,
    productRoot: string,
    native?: WaeNativeAddon | null,
): ProductUpdateStatus {
    const addon = native ?? loadNativeAddon(resolveNativeAbsolutePath(manifest, productRoot));
    return addon.checkProductUpdate(toNativeOptions(manifest, productRoot));
}

export function applyProductUpdateFromManifest(
    manifest: WaeProductManifest,
    productRoot: string,
    native?: WaeNativeAddon | null,
): ProductUpdateStatus {
    const addon = native ?? loadNativeAddon(resolveNativeAbsolutePath(manifest, productRoot));
    return addon.applyProductUpdate(toNativeOptions(manifest, productRoot));
}

export { loadProductManifest };
