import { getElement } from './utils.ts';
import { renderError } from './ui.ts';
import type { InitInput } from '../pkg';

interface WasmModule {
    readonly default: (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>) => Promise<unknown>;
    readonly rsa_demo_text: (text: string, p: bigint, q: bigint) => string;
    readonly rsa_demo_number: (message: bigint, p: bigint, q: bigint) => string;
    readonly sha256_demo_text: (text: string) => string;
}

interface WasmState {
    readonly isReady: () => boolean;
    readonly get: () => WasmModule;
    readonly initialise: (module: WasmModule) => void;
}

const createWasmState = (): WasmState => {
    let module: WasmModule | undefined;
    return {
        isReady: () => module !== undefined,
        get: () => {
            if (!module) throw new Error('WASM not initialised');
            return module;
        },
        initialise: (wasm: WasmModule) => {
            if (module) throw new Error('WASM already initialised');
            module = wasm;
        }
    };
};

const wasmState = createWasmState();

export const isWasmReady = (): boolean => wasmState.isReady();

export const rsaDemoText = (text: string, p: bigint, q: bigint): string => wasmState.get().rsa_demo_text(text, p, q);

export const rsaDemoNumber = (message: bigint, p: bigint, q: bigint): string => wasmState.get().rsa_demo_number(message, p, q);

export const sha256DemoText = (text: string): string => wasmState.get().sha256_demo_text(text);

export const initWasm = async (): Promise<boolean> => {
    try {
        const wasm = await import('../pkg/encryption_demo.js') as WasmModule;
        await wasm.default();
        wasmState.initialise(wasm);
        console.log('WASM loaded successfully!');
        return true;
    } catch (e) {
        console.error('Failed to load WASM:', e);
        const resultsDiv = getElement('rsa-results');
        if (resultsDiv) {
            renderError(resultsDiv, 'Failed to load WebAssembly module. Make sure you\'re running this from a web server.');
        }
        return false;
    }
};

