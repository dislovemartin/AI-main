import { API } from './api.js';
import { Metrics } from './metrics.js';
import { Router } from './router.js';
import { UI } from './ui.js';

class App {
    constructor() {
        this.initializeEventListeners();
    }

    initializeEventListeners() {
        // Navigation
        document.querySelectorAll('.nav-menu a').forEach(link => {
            link.addEventListener('click', (e) => {
                e.preventDefault();
                const route = e.target.getAttribute('href').substring(1);
                Router.navigate(route);
            });
        });

        // Form submissions
        document.addEventListener('submit', this.handleFormSubmit.bind(this));
    }

    async handleFormSubmit(e) {
        if (e.target.matches('.feedback-form')) {
            e.preventDefault();
            const textarea = e.target.querySelector('textarea');
            try {
                await API.post('/feedback', {
                    user_id: 'anonymous', // Replace with actual user ID
                    comments: textarea.value
                });
                UI.showNotification('Feedback submitted successfully', 'success');
                textarea.value = '';
            } catch (error) {
                UI.showNotification(error.message, 'error');
            }
        }
    }

    async start() {
        Metrics.startTiming('appInit');
        await Router.navigate();
        const initTime = Metrics.endTiming('appInit');
        await Metrics.sendMetrics({ initTime });
    }
}

// Initialize the application
document.addEventListener('DOMContentLoaded', () => {
    const app = new App();
    app.start();
}); 