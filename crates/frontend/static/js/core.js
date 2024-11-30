const API = {
    BASE_URL: '/api',
    
    async request(endpoint, options = {}) {
        const response = await fetch(`${this.BASE_URL}${endpoint}`, {
            headers: {
                'Content-Type': 'application/json',
            },
            ...options
        });
        
        if (!response.ok) {
            throw new Error(`API Error: ${response.statusText}`);
        }
        
        return response.json();
    },
    
    async get(endpoint) {
        return this.request(endpoint);
    },
    
    async post(endpoint, data) {
        return this.request(endpoint, {
            method: 'POST',
            body: JSON.stringify(data)
        });
    }
};

const UI = {
    elements: {
        loading: document.getElementById('loading'),
        notification: document.getElementById('notification'),
        content: document.getElementById('content')
    },

    showLoading(message = 'Loading...') {
        this.elements.loading.textContent = message;
        this.elements.loading.classList.remove('hidden');
    },

    hideLoading() {
        this.elements.loading.classList.add('hidden');
    },

    showNotification(message, type = 'info') {
        this.elements.notification.textContent = message;
        this.elements.notification.className = `notification ${type}`;
        this.elements.notification.classList.remove('hidden');
        
        setTimeout(() => {
            this.elements.notification.classList.add('hidden');
        }, 3000);
    }
};

const Metrics = {
    timings: {},
    
    startTiming(label) {
        this.timings[label] = performance.now();
    },
    
    endTiming(label) {
        if (this.timings[label]) {
            const duration = performance.now() - this.timings[label];
            delete this.timings[label];
            return duration;
        }
        return 0;
    },
    
    async sendMetrics(data) {
        try {
            await API.post('/metrics', data);
        } catch (error) {
            console.error('Failed to send metrics:', error);
        }
    }
};

// Initialize the application
document.addEventListener('DOMContentLoaded', async () => {
    try {
        Metrics.startTiming('appInit');
        
        // Load initial data
        UI.showLoading();
        const data = await API.get('/initial-data');
        
        // Render content
        UI.elements.content.innerHTML = `
            <div class="dashboard">
                <h1>Dashboard</h1>
                <div class="metrics">
                    <div class="metric">
                        <h3>Performance</h3>
                        <p>${data.performance}%</p>
                    </div>
                </div>
            </div>
        `;
        
        const initTime = Metrics.endTiming('appInit');
        await Metrics.sendMetrics({ initTime });
        
        UI.hideLoading();
    } catch (error) {
        UI.hideLoading();
        UI.showNotification(error.message, 'error');
    }
}); 