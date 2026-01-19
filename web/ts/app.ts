import { initWasm, runRsaDemo, runSha256Demo } from './demos.ts';
import { getElement, getElements, onEnterKey, onEnterKeyUnlessShift } from './utils.ts';

const switchTab = (event: Event): void => {
    const target = event.target as HTMLElement;
    const tabName = target.dataset.tab;
    getElements('.tab-button').forEach(btn => btn.classList.remove('active'));
    target.classList.add('active');
    getElements('.panel').forEach(panel => panel.classList.add('hidden'));
    getElement(`${tabName}-panel`)?.classList.remove('hidden');
};

const registerClickHandler = (id: string, handler: () => void): void => {
    getElement(id)?.addEventListener('click', handler);
};

const registerEnterKeyHandler = (id: string, handler: () => void, allowShift = false): void => {
    const keyHandler = allowShift ? onEnterKeyUnlessShift(handler) : onEnterKey(handler);
    getElement(id)?.addEventListener('keypress', keyHandler as EventListener);
};

const setupEventListeners = (): void => {
    getElements('.tab-button').forEach(btn => btn.addEventListener('click', switchTab));

    registerClickHandler('rsa-run-btn', () => void runRsaDemo());
    registerClickHandler('sha256-run-btn', () => void runSha256Demo());

    ['rsa-message', 'rsa-p', 'rsa-q'].forEach(id =>
        registerEnterKeyHandler(id, () => void runRsaDemo())
    );
    registerEnterKeyHandler('sha256-message', () => void runSha256Demo(), true);
};

const init = async (): Promise<void> => {
    await initWasm();
    setupEventListeners();
};

void init();

