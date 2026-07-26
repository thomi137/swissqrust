/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "*.html", // Markup lives in index.html now (Alpine.js-driven)
        "**/*.rs",
    ],
    theme: {
        extend: {
            colors: { 'swiss-red': '#D32F2F' }
        },
    },
    plugins: [],
}