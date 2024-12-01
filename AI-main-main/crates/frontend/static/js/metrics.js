export const Metrics = {
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