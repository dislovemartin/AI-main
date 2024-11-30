export const Router = {
    routes: {
        dashboard: async () => {
            const data = await API.get('/initial-data');
            return `
                <div class="dashboard">
                    <div class="card">
                        <h3>Performance</h3>
                        <p>${data.performance}%</p>
                        <div class="chart" id="performanceChart"></div>
                    </div>
                    <div class="card">
                        <h3>Active Users</h3>
                        <p>${data.activeUsers}</p>
                        <div class="chart" id="usersChart"></div>
                    </div>
                    <div class="card">
                        <h3>System Status</h3>
                        <p class="status ${data.status.toLowerCase()}">${data.status}</p>
                    </div>
                </div>
            `;
        },
        chat: () => `
            <div class="chat-container">
                <div class="chat-messages" id="chatMessages"></div>
                <form class="chat-input" id="chatForm">
                    <input type="text" placeholder="Type your message..." required>
                    <button type="submit">Send</button>
                </form>
            </div>
        `,
        feedback: () => `
            <div class="feedback-container">
                <h2>Feedback</h2>
                <form class="feedback-form">
                    <textarea 
                        placeholder="Your feedback helps us improve..." 
                        required
                    ></textarea>
                    <button type="submit">Submit Feedback</button>
                </form>
            </div>
        `
    },

    async navigate(route = 'dashboard') {
        const routeHandler = this.routes[route];
        if (routeHandler) {
            UI.showLoading();
            try {
                const content = await routeHandler();
                UI.renderContent(content);
                this.updateActiveLink(route);
                window.location.hash = route;
            } catch (error) {
                UI.showNotification(error.message, 'error');
            } finally {
                UI.hideLoading();
            }
        }
    },

    updateActiveLink(route) {
        document.querySelectorAll('.nav-link').forEach(link => {
            link.classList.toggle('active', link.getAttribute('href') === `#${route}`);
        });
    }
}; 