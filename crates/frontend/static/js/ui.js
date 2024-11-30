export const UI = {
    elements: {
        loading: document.getElementById('loading'),
        notification: document.getElementById('notification'),
        content: document.getElementById('content')
    },

    showLoading(message = 'Loading...') {
        this.elements.loading.innerHTML = `
            <div class="loading">
                <div class="loading-spinner"></div>
                <p>${message}</p>
            </div>
        `;
        this.elements.loading.classList.remove('hidden');
    },

    hideLoading() {
        this.elements.loading.classList.add('hidden');
    },

    showNotification(message, type = 'info') {
        this.elements.notification.innerHTML = `
            <div class="notification ${type}">
                <p>${message}</p>
                <button class="close-btn" onclick="this.parentElement.classList.add('hidden')">✕</button>
            </div>
        `;
        this.elements.notification.classList.remove('hidden');
        
        setTimeout(() => {
            this.elements.notification.classList.add('hidden');
        }, 5000);
    },

    renderContent(html) {
        this.elements.content.innerHTML = html;
    }
}; 