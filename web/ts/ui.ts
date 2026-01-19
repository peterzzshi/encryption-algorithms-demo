export interface RsaKeyPair {
    readonly n: string;
    readonly e: string;
    readonly d: string;
    readonly p: string;
    readonly q: string;
    readonly phi_n: string;
}

export interface RsaStep {
    readonly step_number: number;
    readonly title: string;
    readonly description: string;
    readonly formula: string;
    readonly result: string;
}

export interface RsaResult {
    readonly key_pair?: RsaKeyPair;
    readonly steps: readonly RsaStep[];
    readonly success: boolean;
    readonly original_message: string;
    readonly decrypted_text?: string;
}

export interface Sha256Step {
    readonly step_number: number;
    readonly title: string;
    readonly description: string;
    readonly data?: readonly string[];
}

export interface Sha256Result {
    readonly hash: string;
    readonly steps: readonly Sha256Step[];
}

const renderStepCard = (stepNumber: number, title: string, description: string, content: string): string => `
    <div class="step-card">
        <div class="step-title">
            <span class="step-number">${stepNumber}</span>
            ${title}
        </div>
        <div class="step-description">${description}</div>
        ${content}
    </div>
`;

const renderKeyCard = (icon: string, title: string, values: string[]): string => `
    <div class="key-card">
        <h4>${icon} ${title}</h4>
        ${values.map(v => `<div class="key-value">${v}</div>`).join('')}
    </div>
`;

const renderKeyPairDisplay = ({ n, e, d, p, q, phi_n }: RsaKeyPair): string => `
    <div class="key-display">
        ${renderKeyCard('🔓', 'Public Key', [`n = ${n}`, `e = ${e}`])}
        ${renderKeyCard('🔐', 'Private Key', [`n = ${n}`, `d = ${d}`])}
        ${renderKeyCard('📊', 'Parameters', [`p = ${p}`, `q = ${q}`, `φ(n) = ${phi_n}`])}
    </div>
`;

const renderRsaStep = ({ step_number, title, description, formula, result }: RsaStep): string =>
    renderStepCard(step_number, title, description, `
        <div class="step-formula">${formula}</div>
        <div class="step-result">Result: ${result}</div>
    `);

const renderSha256Step = ({ step_number, title, description, data = [] }: Sha256Step): string => {
    const dataHtml = data.length > 0
        ? `<div class="step-data">${data.map(item => `<div class="step-data-item">${item}</div>`).join('')}</div>`
        : '';
    return renderStepCard(step_number, title, description, dataHtml);
};

const renderMessage = (className: string, icon: string, message: string): string =>
    `<div class="${className}">${icon} ${message}</div>`;


export const buildRsaResultsHtml = ({ key_pair, steps, success, original_message, decrypted_text }: RsaResult): string => {
    const keyPairHtml = key_pair ? renderKeyPairDisplay(key_pair) : '';
    const stepsHtml = steps.map(renderRsaStep).join('');
    const successHtml = success
        ? renderMessage('success-message', '✅',
            `Success! Original message "${original_message}" was encrypted and decrypted correctly.` +
            (decrypted_text ? `<br>Decrypted text: "${decrypted_text}"` : ''))
        : '';
    return keyPairHtml + stepsHtml + successHtml;
};

export const buildSha256ResultsHtml = ({ hash, steps }: Sha256Result): string => {
    const hashHtml = `<div class="hash-result"><strong>SHA-256 Hash:</strong><br>${hash}</div>`;
    const stepsHtml = steps.map(renderSha256Step).join('');
    const successHtml = renderMessage('success-message', '✅', 'SHA-256 hash computed successfully!');
    return hashHtml + stepsHtml + successHtml;
};

export const buildErrorHtml = (message: string): string =>
    renderMessage('error-message', '❌', message);

export const buildLoadingHtml = (message: string): string =>
    renderMessage('loading', '⏳', message);


export const renderToContainer = (container: HTMLElement, html: string): void => {
    container.innerHTML = html;
};


export const renderRsaResults = (container: HTMLElement, result: RsaResult): void =>
    renderToContainer(container, buildRsaResultsHtml(result));

export const renderSha256Results = (container: HTMLElement, result: Sha256Result): void =>
    renderToContainer(container, buildSha256ResultsHtml(result));

export const renderError = (container: HTMLElement, message: string): void =>
    renderToContainer(container, buildErrorHtml(message));

export const renderLoading = (container: HTMLElement, message: string): void =>
    renderToContainer(container, buildLoadingHtml(message));

