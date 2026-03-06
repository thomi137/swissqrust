/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "**/*.rs", // This looks at your main.rs in the same folder
    ],
    theme: {
        extend: {
            colors: { 'swiss-red': '#D32F2F' }
        },
    },
    plugins: [],
}