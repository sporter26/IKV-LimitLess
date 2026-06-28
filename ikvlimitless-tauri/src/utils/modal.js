export const modalEventEmitter = new EventTarget();

export const showModal = (message, type = 'info') => {
  modalEventEmitter.dispatchEvent(new CustomEvent('showModal', { detail: { message, type } }));
};

export const toast = {
  success: (msg) => showModal(msg, 'success'),
  error: (msg) => showModal(msg, 'error'),
  info: (msg) => showModal(msg, 'info')
};
