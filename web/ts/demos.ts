import { initWasm as initWasmModule, isWasmReady, rsaDemoText, rsaDemoNumber, sha256DemoText } from './wasm.ts';
import { renderRsaResults, renderSha256Results, renderError, renderLoading } from './ui.ts';
import type { RsaResult, Sha256Result } from './ui.ts';
import { getElement, getTrimmedValue, getValue, parseIntSafe, isNumericString, validateNonEmpty, validateNumbers, parseResult } from './utils.ts';
import type { ValidationResult, ParsedResult } from './utils.ts';

export { initWasmModule as initWasm };

interface RsaInputs {
    readonly message: string;
    readonly p: number | null;
    readonly q: number | null;
    readonly resultsDiv: HTMLElement | null;
}

interface Sha256Inputs {
    readonly message: string;
    readonly resultsDiv: HTMLElement | null;
}

interface DemoConfig<TInputs, TResult> {
    readonly getInputs: () => TInputs;
    readonly validate: (inputs: TInputs) => ValidationResult;
    readonly compute: (inputs: TInputs) => ParsedResult<TResult>;
    readonly renderSuccess: (container: HTMLElement, result: TResult) => void;
    readonly loadingMessage: string;
}

const getRsaInputs = (): RsaInputs => ({
    message: getTrimmedValue('rsa-message'),
    p: parseIntSafe(getValue('rsa-p')),
    q: parseIntSafe(getValue('rsa-q')),
    resultsDiv: getElement('rsa-results')
});

const getSha256Inputs = (): Sha256Inputs => ({
    message: getValue('sha256-message'),
    resultsDiv: getElement('sha256-results')
});

const validateRsaInputs = ({ message, p, q }: RsaInputs): ValidationResult => {
    const messageCheck = validateNonEmpty(message, 'a message');
    if (!messageCheck.valid) return messageCheck;
    return validateNumbers(p, q);
};

const validateSha256Inputs = ({ message }: Sha256Inputs): ValidationResult =>
    validateNonEmpty(message, 'a message');

const computeRsa = (message: string, p: number, q: number): ParsedResult<RsaResult> => {
    const resultJson = isNumericString(message)
        ? rsaDemoNumber(BigInt(message), BigInt(p), BigInt(q))
        : rsaDemoText(message, BigInt(p), BigInt(q));
    return parseResult<RsaResult>(resultJson);
};

const computeSha256 = (message: string): ParsedResult<Sha256Result> => {
    const resultJson = sha256DemoText(message);
    return parseResult<Sha256Result>(resultJson);
};

const runDemo = async <TInputs extends { resultsDiv: HTMLElement | null }, TResult>({
    getInputs,
    validate,
    compute,
    renderSuccess,
    loadingMessage
}: DemoConfig<TInputs, TResult>): Promise<void> => {
    if (!isWasmReady()) {
        alert('WebAssembly is still loading. Please wait.');
        return;
    }

    const inputs = getInputs();
    const { resultsDiv } = inputs;

    if (!resultsDiv) {
        console.error('Results container not found');
        return;
    }

    const validation = validate(inputs);
    if (!validation.valid) {
        renderError(resultsDiv, validation.error);
        return;
    }

    renderLoading(resultsDiv, loadingMessage);

    try {
        const result = compute(inputs);
        if (!result.ok) {
            renderError(resultsDiv, result.error);
            return;
        }
        renderSuccess(resultsDiv, result.data);
    } catch (e) {
        console.error(e);
        renderError(resultsDiv, e instanceof Error ? e.message : 'An unknown error occurred');
    }
};

export const runRsaDemo = (): Promise<void> => runDemo<RsaInputs, RsaResult>({
    getInputs: getRsaInputs,
    validate: validateRsaInputs,
    compute: ({ message, p, q }) => computeRsa(message, p!, q!),
    renderSuccess: renderRsaResults,
    loadingMessage: 'Processing...'
});

export const runSha256Demo = (): Promise<void> => runDemo<Sha256Inputs, Sha256Result>({
    getInputs: getSha256Inputs,
    validate: validateSha256Inputs,
    compute: ({ message }) => computeSha256(message),
    renderSuccess: renderSha256Results,
    loadingMessage: 'Computing hash...'
});

