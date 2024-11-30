export const Theme = {
    init() {
        this.themeToggle = document.getElementById('theme-toggle');
        this.currentTheme = localStorage.getItem('theme') || 'light';
        this.applyTheme(this.currentTheme);
        this.bindEvents();
    },

    bindEvents() {
        this.themeToggle.addEventListener('click', () => {
            this.currentTheme = this.currentTheme === 'light' ? 'dark' : 'light';
            this.applyTheme(this.currentTheme);
            localStorage.setItem('theme', this.currentTheme);
        });
    },

    applyTheme(theme) {
        document.body.setAttribute('data-theme', theme);
        this.themeToggle.querySelector('i').className = 
            `icon-theme-${theme === 'light' ? 'dark' : 'light'}`;
    }
}; 