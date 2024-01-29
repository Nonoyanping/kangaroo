// static/script.js

async function fetchSentence() {
    const response = await fetch('/get_sentence');
    const sentence = await response.text();
    document.getElementById('sentence-container').textContent = sentence;
}

function toggleTheme() {
    document.body.classList.toggle('dark-theme');
    document.getElementById('sentence-container').classList.toggle('dark-theme');
}

document.addEventListener('DOMContentLoaded', () => {
    fetchSentence();

    // Example: Toggle theme on button click
    document.getElementById('toggle-theme-btn').addEventListener('click', toggleTheme);
});
